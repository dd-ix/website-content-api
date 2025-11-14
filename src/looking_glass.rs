use crate::cache::{Cache, Updater};
use ipnet::IpNet;
use reqwest::Client;
use serde::Deserialize;
use std::net::IpAddr;
use std::sync::Arc;
use tracing::{error, info};
use url::Url;

#[derive(Clone)]
pub(crate) struct LookingGlassUpdater {
  looking_glass_url: Url,
  client: Client,
}

#[derive(Deserialize)]
pub(crate) struct LookingGlassNeighbor {
  asn: i32,
}

#[derive(Deserialize)]
pub(crate) struct LookingGlassNeighbors {
  neighbors: Vec<LookingGlassNeighbor>,
}

impl LookingGlassUpdater {
  pub(crate) async fn load(looking_glass_url: Url) -> anyhow::Result<Self> {
    Ok(Self {
      looking_glass_url,
      client: Default::default(),
    })
  }
}

impl Updater for LookingGlassUpdater {
  type Output = Vec<IpNet>;
  type Error = anyhow::Error;

  async fn update(&self) -> Result<Self::Output, Self::Error> {
    let asns: Vec<i32> = self
      .client
      .get(
        self
          .looking_glass_url
          .join("/api/v1/routeservers/rs01_v4/neighbors")?,
      )
      .send()
      .await?
      .error_for_status()?
      .json::<LookingGlassNeighbors>()
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
            .get(self.looking_glass_url.join(&format!(
              "/api/v1/routeservers/rs01_{}/neighbors/AS{}_1/routes/received?pf={}",
              inet_type, asn, current_page
            ))?)
            .send()
            .await?
            .error_for_status()
          {
            Ok(response) => {
              let json_data = response.json::<LookingGlassRoutesScheme>().await?;

              total_number_of_pages = json_data.pagination.total_pages;
              let mut route_array: Vec<IpNet> = json_data
                .imported
                .into_iter()
                .map(|looking_glass_import: LookingGlassImport| looking_glass_import.network)
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
  pub routes: Arc<Cache<LookingGlassUpdater>>,
}

#[derive(Deserialize)]
struct LookingGlassPagination {
  total_pages: u32,
}

#[derive(Deserialize, Clone)]
struct LookingGlassImport {
  network: IpNet,
}

#[derive(Deserialize)]
struct LookingGlassRoutesScheme {
  pagination: LookingGlassPagination,
  imported: Vec<LookingGlassImport>,
}

impl LookingGlass {
  pub(crate) async fn load(looking_glass_url: Url) -> anyhow::Result<Self> {
    Ok(Self {
      routes: Arc::new(Cache::new(
        LookingGlassUpdater::load(looking_glass_url).await?,
      )),
    })
  }
}

pub fn is_address_in_network(routes: &[IpNet], network: IpAddr) -> bool {
  routes.iter().any(|x| x.contains(&network))
}
