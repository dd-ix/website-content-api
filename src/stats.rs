use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use anyhow::anyhow;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use tokio::sync::RwLock;
use tokio::try_join;
use url::Url;

const MAX_AGE: Duration = Duration::minutes(10);

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
  prometheus_url: Arc<Url>,
  client: Client,
  updating: Arc<AtomicBool>,
  cached: Arc<RwLock<Option<(OffsetDateTime, Arc<AggregatedStats>)>>>,
}

impl Stats {
  pub(crate) fn new(prometheus_url: Url) -> Self {
    Self {
      prometheus_url: Arc::new(prometheus_url),
      client: Client::new(),
      updating: Arc::new(AtomicBool::new(false)),
      cached: Arc::new(RwLock::new(Option::None)),
    }
  }

  pub(crate) async fn get_stats(&self) -> anyhow::Result<Arc<AggregatedStats>> {
    let now = OffsetDateTime::now_utc();
    {
      while self.updating.load(Ordering::Relaxed) {
        tokio::time::sleep(std::time::Duration::from_millis(500)).await
      }

      let lock = self.cached.read().await;
      if let Some((next_update, stats)) = lock.as_ref() {
        if next_update < &now {
          self.updating.store(true, Ordering::Relaxed);
        } else {
          return Ok(stats.clone());
        }
      }
    }

    let (two_days, seven_days, month) = try_join!(
      self.query_stats(OffsetDateTime::now_utc() - Duration::days(2), now, 255.0),
      self.query_stats(OffsetDateTime::now_utc() - Duration::days(7), now, 255.0),
      self.query_stats(OffsetDateTime::now_utc() - Duration::days(30), now, 255.0),
    )?;

    let metrics = Arc::new(AggregatedStats {
      two_days,
      seven_days,
      month,
    });

    *self.cached.write().await = Some((now + MAX_AGE, metrics.clone()));
    self.updating.store(false, Ordering::Relaxed);

    Ok(metrics)
  }

  async fn query_stats(
    &self,
    start: OffsetDateTime,
    end: OffsetDateTime,
    points: f64,
  ) -> anyhow::Result<Series> {
    let query = PrometheusQuery {
      query: "sum(rate(sflow_agent_bytes[5m]))*8".to_string(),
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
