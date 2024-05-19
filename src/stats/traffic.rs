use std::str::FromStr;

use anyhow::anyhow;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use url::Url;

use crate::cache::Updater;

use super::{Series, TimeSelection};

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

pub(super) struct TrafficUpdater {
  client: Client,
  prometheus_url: Url,
  selection: Duration,
}

#[async_trait::async_trait]
impl Updater for TrafficUpdater {
  type Output = Series;
  type Error = anyhow::Error;

  async fn update(&self) -> Result<Self::Output, Self::Error> {
    let now = OffsetDateTime::now_utc();

    let data = self
      .query_stats(OffsetDateTime::now_utc() - self.selection, now, 255.0)
      .await?;

    Ok(data)
  }
}

impl TrafficUpdater {
  pub(super) fn new(client: Client, prometheus_url: Url, selection: TimeSelection) -> Self {
    Self {
      client,
      prometheus_url,
      selection: selection.into(),
    }
  }

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
