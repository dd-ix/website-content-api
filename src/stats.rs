use std::str::FromStr;
use std::sync::Arc;

use anyhow::anyhow;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use url::Url;

use crate::cache::{Cache, Updater};

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum TimeSelection {
  LastTwoDays,
  LastWeek,
  LastMonth,
  LastThreeMonths,
  LastYear,
}

impl From<TimeSelection> for Duration {
  fn from(value: TimeSelection) -> Self {
    match value {
      TimeSelection::LastTwoDays => Duration::days(2),
      TimeSelection::LastWeek => Duration::weeks(1),
      TimeSelection::LastMonth => Duration::days(30),
      TimeSelection::LastThreeMonths => Duration::days(90),
      TimeSelection::LastYear => Duration::days(365),
    }
  }
}

struct TimeSelectionStore<T> {
  two_days: T,
  week: T,
  month: T,
  three_months: T,
  year: T,
}

impl<T> TimeSelectionStore<T> {
  pub(crate) fn get(&self, selection: TimeSelection) -> &T {
    match selection {
      TimeSelection::LastTwoDays => &self.two_days,
      TimeSelection::LastWeek => &self.week,
      TimeSelection::LastMonth => &self.month,
      TimeSelection::LastThreeMonths => &self.three_months,
      TimeSelection::LastYear => &self.year,
    }
  }
}

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
pub(crate) struct Series {
  #[serde(with = "time::serde::rfc3339")]
  start: OffsetDateTime,
  #[serde(with = "time::serde::rfc3339")]
  end: OffsetDateTime,
  data: Vec<(f64, f64)>,
}

#[derive(Clone)]
pub(crate) struct Stats {
  traffic: Arc<TimeSelectionStore<Cache<TrafficUpdater>>>,
  as112: Arc<TimeSelectionStore<Cache<As112Updater>>>,
}

struct TrafficUpdater {
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

struct As112Updater {
  client: Client,
  prometheus_url: Url,
  selection: Duration,
}

#[async_trait::async_trait]
impl Updater for As112Updater {
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
impl As112Updater {
  async fn query_stats(
    &self,
    start: OffsetDateTime,
    end: OffsetDateTime,
    points: f64,
  ) -> anyhow::Result<Series> {
    let query = PrometheusQuery {
      query: "sum(rate(knot_query_type_total[5m]))".to_string(),
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

    Self {
      traffic: Arc::new(TimeSelectionStore {
        two_days: Cache::new(TrafficUpdater {
          client: client.clone(),
          prometheus_url: prometheus_url.clone(),
          selection: TimeSelection::LastTwoDays.into(),
        }),
        week: Cache::new(TrafficUpdater {
          client: client.clone(),
          prometheus_url: prometheus_url.clone(),
          selection: TimeSelection::LastWeek.into(),
        }),
        month: Cache::new(TrafficUpdater {
          client: client.clone(),
          prometheus_url: prometheus_url.clone(),
          selection: TimeSelection::LastMonth.into(),
        }),
        three_months: Cache::new(TrafficUpdater {
          client: client.clone(),
          prometheus_url: prometheus_url.clone(),
          selection: TimeSelection::LastThreeMonths.into(),
        }),
        year: Cache::new(TrafficUpdater {
          client: client.clone(),
          prometheus_url: prometheus_url.clone(),
          selection: TimeSelection::LastYear.into(),
        }),
      }),
      as112: Arc::new(TimeSelectionStore {
        two_days: Cache::new(As112Updater {
          client: client.clone(),
          prometheus_url: prometheus_url.clone(),
          selection: TimeSelection::LastTwoDays.into(),
        }),
        week: Cache::new(As112Updater {
          client: client.clone(),
          prometheus_url: prometheus_url.clone(),
          selection: TimeSelection::LastWeek.into(),
        }),
        month: Cache::new(As112Updater {
          client: client.clone(),
          prometheus_url: prometheus_url.clone(),
          selection: TimeSelection::LastMonth.into(),
        }),
        three_months: Cache::new(As112Updater {
          client: client.clone(),
          prometheus_url: prometheus_url.clone(),
          selection: TimeSelection::LastThreeMonths.into(),
        }),
        year: Cache::new(As112Updater {
          client: client.clone(),
          prometheus_url: prometheus_url.clone(),
          selection: TimeSelection::LastYear.into(),
        }),
      }),
    }
  }

  pub(crate) async fn get_traffic_stats(
    &self,
    selection: TimeSelection,
  ) -> anyhow::Result<Arc<Series>> {
    self.traffic.get(selection).get().await
  }

  pub(crate) async fn get_as112_stats(
    &self,
    selection: TimeSelection,
  ) -> anyhow::Result<Arc<Series>> {
    self.as112.get(selection).get().await
  }
}
