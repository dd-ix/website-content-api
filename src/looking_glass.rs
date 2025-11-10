use crate::cache::{Cache, Updater};
use ipnet::IpNet;
use reqwest::Client;
use serde::Deserialize;
use std::net::IpAddr;
use std::sync::Arc;
use tracing::{error, info};
use url::Url;

#[derive(Clone)]
pub(crate) struct AliceLookingGlassUpdater {
  looking_glass_address: Url,
  client: Client,
}

#[derive(Deserialize)]
pub(crate) struct AliceLookingGlassNeighbor {
  asn: i32,
}

#[derive(Deserialize)]
pub(crate) struct AliceLookingGlassNeighbors {
  neighbors: Vec<AliceLookingGlassNeighbor>,
}

impl AliceLookingGlassUpdater {
  pub(crate) async fn load(looking_glass_address: Url) -> anyhow::Result<Self> {
    Ok(Self {
      looking_glass_address,
      client: Default::default(),
    })
  }
}

impl Updater for AliceLookingGlassUpdater {
  type Output = Vec<IpNet>;
  type Error = anyhow::Error;

  async fn update(&self) -> Result<Self::Output, Self::Error> {
    let asns: Vec<i32> = self
      .client
      .get(format!(
        "https://{}/api/v1/routeservers/rs01_v4/neighbors",
        self.looking_glass_address
      ))
      .send()
      .await?
      .error_for_status()?
      .json::<AliceLookingGlassNeighbors>()
      .await?
      .neighbors
      .iter()
      .map(|neighbor| neighbor.asn)
      .collect();

    let mut routes = Vec::new();

    for inet_type in ["v4", "v6"] {
      for asn in &asns {
        let mut total_number_of_pages = 1;
        let mut current_page = 0;
        while current_page < total_number_of_pages {
          info!(
            "fetching {inet_type} routes for asn {asn}: page {current_page}/{total_number_of_pages}",
          );
          match self
            .client
            .get(format!(
              "https://{}/api/v1/routeservers/rs01_{}/neighbors/AS{}_1/routes/received?pf={}",
              self.looking_glass_address, inet_type, asn, current_page
            ))
            .send()
            .await?
            .error_for_status()
          {
            Ok(response) => {
              let json_data = response.json::<AliceLookingGlassRoutesScheme>().await?;

              total_number_of_pages = json_data.pagination.total_pages;
              let mut route_array: Vec<IpNet> = json_data
                .imported
                .into_iter()
                .map(|alice_looking_glass_import: AliceLookingGlassImport| {
                  alice_looking_glass_import.network
                })
                .collect();
              routes.append(&mut route_array);
            }
            Err(e) => {
              error!("{e}");
            }
          }
          current_page += 1;
        }
      }
    }

    Ok(routes)
  }
}

#[derive(Clone)]
pub struct LookingGlass {
  pub routes: Arc<Cache<AliceLookingGlassUpdater>>,
}

#[derive(Deserialize)]
struct AliceLookingGlassPagination {
  total_pages: u32,
}

#[derive(Deserialize, Clone)]
struct AliceLookingGlassImport {
  network: IpNet,
}

#[derive(Deserialize)]
struct AliceLookingGlassRoutesScheme {
  pagination: AliceLookingGlassPagination,
  imported: Vec<AliceLookingGlassImport>,
}

impl LookingGlass {
  pub(crate) async fn load(looking_glass_url: Url) -> anyhow::Result<Self> {
    Ok(Self {
      routes: Arc::new(Cache::new(
        AliceLookingGlassUpdater::load(looking_glass_url).await?,
      )),
    })
  }
}

pub fn is_address_in_network(routes: &[IpNet], network: IpAddr) -> bool {
  routes.iter().any(|x| x.contains(&network))
}
