use std::collections::HashMap;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use tokio::sync::RwLock;
use tracing::{error, info};
use url::Url;

use crate::cache::Cache;
use crate::cache::Updater;

// https://github.com/euro-ix/json-schemas/wiki/Schema-Field-Entries-Members#schema-field-entries---members

const MAX_AGE: Duration = Duration::hours(1);

#[derive(Deserialize, PartialEq, Clone)]
enum EuroIXMemberType {
  #[serde(rename = "peering")]
  Peering,
  #[serde(rename = "ixp")]
  IXP,
  #[serde(rename = "other")]
  Other,
}

#[derive(Deserialize, Serialize, Clone)]
enum PeeringPolicy {
  #[serde(rename = "open")]
  Open,
  #[serde(rename = "selective")]
  Selective,
  #[serde(rename = "case-by-case")]
  CaseByCase,
  #[serde(rename = "mandatory")]
  Mandatory,
}

#[derive(Deserialize, Clone)]
struct EuroIXIfList {
  if_speed: u64,
}

#[derive(Deserialize, Clone)]
struct EuroIXVLanList {
  ipv4: Option<serde_json::Value>,
  ipv6: Option<serde_json::Value>,
}

#[derive(Deserialize, Clone)]
struct EuroIXConnection {
  if_list: Vec<EuroIXIfList>,
  vlan_list: Vec<EuroIXVLanList>,
}

#[derive(Deserialize, Clone)]
struct EuroIXMemberScheme {
  asnum: i32,
  member_type: EuroIXMemberType,
  name: String,
  url: Url,
  peering_policy: PeeringPolicy,
  connection_list: Vec<EuroIXConnection>,
}

#[derive(Deserialize, Clone)]
struct EuroIXApiScheme {
  member_list: Vec<EuroIXMemberScheme>,
}

#[derive(Deserialize, Clone)]
struct Supporter {
  name: String,
  url: Url,
}

#[derive(Deserialize, Clone)]
struct StaticSupporterInformation {
  supporters: Vec<Supporter>,
  supporting_peers: Vec<i32>,
}

#[derive(Serialize, Clone)]
struct ConnectionSpeed {
  speed: u64,
  amount: u64,
}

#[derive(Serialize, Clone)]
pub(crate) struct FoundationEntity {
  supporter: bool,
  rs_v4: bool,
  rs_v6: bool,
  asn: Option<i32>,
  name: String,
  url: Url,
  speed: Vec<ConnectionSpeed>,
}

#[derive(Clone)]
pub(crate) struct NetworkService {
  cached: Arc<Cache<PeersUpdater>>,
}

struct PeersUpdater {
  client: Client,
  ixp_manager_url: Url,
  yaml_file: StaticSupporterInformation,
}

#[async_trait::async_trait]
impl Updater for PeersUpdater {
  type Output = Vec<FoundationEntity>;
  type Error = anyhow::Error;

  async fn update(&self) -> Result<Self::Output, Self::Error> {
    info!("updating member & supporter list!");
    let api_result: EuroIXApiScheme = self
      .client
      .get(self.ixp_manager_url.join("/api/v4/member-export/ixf/0.6")?)
      .send()
      .await?
      .error_for_status()?
      .json::<crate::peers::EuroIXApiScheme>()
      .await?;

    let mut peers: Vec<FoundationEntity> = api_result
      .member_list
      .into_iter()
      .filter(|peer| peer.member_type != EuroIXMemberType::IXP)
      .map(|value| {
        let is_supporter = self.yaml_file.supporting_peers.contains(&value.asnum);
        let mut does_v4 = false;
        let mut does_v6 = false;
        let mut speeds: HashMap<u64, u64> = HashMap::new();

        for connection_list in value.connection_list {
          for if_list in connection_list.if_list {
            speeds
              .entry(if_list.if_speed)
              .and_modify(|count| *count += 1)
              .or_insert(1);
          }
          for vlan in connection_list.vlan_list {
            does_v4 = does_v4 || vlan.ipv4.is_some();
            does_v6 = does_v6 || vlan.ipv6.is_some();
          }
        }

        let speed_list: Vec<ConnectionSpeed> = speeds
          .into_iter()
          .map(|(key, value)| ConnectionSpeed {
            speed: key,
            amount: value,
          })
          .collect();

        FoundationEntity {
          supporter: is_supporter,
          rs_v4: does_v4,
          rs_v6: does_v6,
          asn: Some(value.asnum),
          name: value.name,
          url: value.url,
          speed: speed_list,
        }
      })
      .collect();

    let mut supporters: Vec<FoundationEntity> = self
      .yaml_file
      .supporters
      .iter()
      .map(|value| FoundationEntity {
        supporter: true,
        rs_v4: false,
        rs_v6: false,
        asn: None,
        name: value.name.clone(),
        url: value.url.clone(),
        speed: Vec::new(),
      })
      .collect();

    peers.append(&mut supporters);

    info!("found {} members & supporters", peers.len());
    peers.sort_by_key(|x| x.name.clone());

    Ok(peers)
  }
}

impl NetworkService {
  pub(crate) async fn new(base_path: &PathBuf, ixp_manager_url: Url) -> anyhow::Result<Self> {
    let serialized_supporter = tokio::fs::read_to_string(base_path.join("supporter.yaml")).await?;
    let yaml_file = serde_yaml::from_str(&serialized_supporter)?;

    let updater = PeersUpdater {
      client: Client::new(),
      ixp_manager_url,
      yaml_file,
    };

    Ok(Self {
      cached: Arc::new(Cache::new(updater)),
    })
  }
  pub(crate) async fn get_stats(&self) -> anyhow::Result<Arc<Vec<FoundationEntity>>> {
    self.cached.get().await
  }
}
