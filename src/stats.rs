use std::str::FromStr;
use std::sync::Arc;

use anyhow::anyhow;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use tokio::try_join;
use url::Url;

use crate::cache::{Cache, Updater};

#[derive(Serialize)]
struct PrometheusQuery {
  query: String,
  #[serde(with = "time::serde::rfc3339")]
  start: OffsetDateTime,
  #[serde(with = "time::serde::rfc3339")]
  end: OffsetDateTime,
  step: f64,
}

#[derive(Deserialize)]
struct PrometheusResponse {
  data: PrometheusData,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PrometheusData {
  result: Vec<PrometheusMetrics>,
}

#[derive(Deserialize)]
struct PrometheusMetrics {
  values: Vec<(f64, String)>,
}

#[derive(Serialize)]
struct Series {
  #[serde(with = "time::serde::rfc3339")]
  start: OffsetDateTime,
  #[serde(with = "time::serde::rfc3339")]
  end: OffsetDateTime,
  data: Vec<(f64, f64)>,
}

#[derive(Serialize)]
pub(crate) struct AggregatedStats {
  two_days: Series,
  seven_days: Series,
  month: Series,
}

#[derive(Clone)]
pub(crate) struct Stats {
  traffic: Arc<Cache<TrafficUpdater>>,
}

struct TrafficUpdater {
  client: Client,
  prometheus_url: Url,
}

#[async_trait::async_trait]
impl Updater for TrafficUpdater {
  type Output = AggregatedStats;
  type Error = anyhow::Error;

  async fn update(&self) -> Result<Self::Output, Self::Error> {
    let now = OffsetDateTime::now_utc();

    let (two_days, seven_days, month) = try_join!(
      self.query_stats(OffsetDateTime::now_utc() - Duration::days(2), now, 255.0),
      self.query_stats(OffsetDateTime::now_utc() - Duration::days(7), now, 255.0),
      self.query_stats(OffsetDateTime::now_utc() - Duration::days(30), now, 255.0),
    )?;

    let metrics = AggregatedStats {
      two_days,
      seven_days,
      month,
    };

    Ok(metrics)
  }
}
impl TrafficUpdater {
  async fn query_stats(
    &self,
    start: OffsetDateTime,
    end: OffsetDateTime,
    points: f64,
  ) -> anyhow::Result<Series> {
    let query = PrometheusQuery {
      query: "sum(rate(sflow_router_bytes[5m]))*8".to_string(),
      start,
      end,
      step: ((end - start) / points).as_seconds_f64(),
    };

    Ok(Series {
      start,
      end,
      data: self
        .client
        .get(self.prometheus_url.join("/api/v1/query_range")?)
        .query(&query)
        .send()
        .await?
        .error_for_status()?
        .json::<PrometheusResponse>()
        .await?
        .data
        .result
        .into_iter()
        .find(|_| true)
        .ok_or_else(|| anyhow!("unexpected prometheus response"))?
        .values
        .into_iter()
        .map(|(time, value)| (time, f64::from_str(&value).unwrap()))
        .collect::<Vec<_>>(),
    })
  }
}

impl Stats {
  pub(crate) fn new(prometheus_url: Url) -> Self {
    let client = Client::new();

    let traffic_updater = TrafficUpdater {
      client,
      prometheus_url,
    };

    Self {
      traffic: Arc::new(Cache::new(traffic_updater)),
    }
  }

  pub(crate) async fn get_stats(&self) -> anyhow::Result<Arc<AggregatedStats>> {
    self.traffic.get().await
  }
}
