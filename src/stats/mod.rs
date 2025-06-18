mod as112;
mod traffic;

use std::{collections::HashMap, sync::Arc};

use reqwest::Client;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use time::{Duration, OffsetDateTime};
use url::Url;

use crate::auto_cache::Cache;

use self::{as112::As112Updater, traffic::TrafficUpdater};

#[derive(Deserialize, EnumIter, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub(crate) enum TimeSelection {
  TwoDays,
  Week,
  Month,
  ThreeMonths,
  Year,
}

impl From<TimeSelection> for Duration {
  fn from(value: TimeSelection) -> Self {
    match value {
      TimeSelection::TwoDays => Duration::days(2),
      TimeSelection::Week => Duration::weeks(1),
      TimeSelection::Month => Duration::days(30),
      TimeSelection::ThreeMonths => Duration::days(90),
      TimeSelection::Year => Duration::days(365),
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

#[derive(Serialize)]
pub(crate) struct Series<T> {
  #[serde(with = "time::serde::rfc3339")]
  start: OffsetDateTime,
  #[serde(with = "time::serde::rfc3339")]
  end: OffsetDateTime,
  data: T,
}

impl<T> TimeSelectionStore<T> {
  pub(crate) fn get(&self, selection: TimeSelection) -> &T {
    match selection {
      TimeSelection::TwoDays => &self.two_days,
      TimeSelection::Week => &self.week,
      TimeSelection::Month => &self.month,
      TimeSelection::ThreeMonths => &self.three_months,
      TimeSelection::Year => &self.year,
    }
  }
}

#[derive(Clone)]
pub(crate) struct Stats {
  traffic: Arc<TimeSelectionStore<Cache<TrafficUpdater>>>,
  as112: Arc<TimeSelectionStore<Cache<As112Updater>>>,
}

impl Stats {
  pub(crate) fn new(prometheus_url: Url) -> Self {
    let client = Client::new();

    Self {
      traffic: Arc::new(TimeSelectionStore {
        two_days: Cache::new(TrafficUpdater::new(
          client.clone(),
          prometheus_url.clone(),
          TimeSelection::TwoDays,
        )),
        week: Cache::new(TrafficUpdater::new(
          client.clone(),
          prometheus_url.clone(),
          TimeSelection::Week,
        )),
        month: Cache::new(TrafficUpdater::new(
          client.clone(),
          prometheus_url.clone(),
          TimeSelection::Month,
        )),
        three_months: Cache::new(TrafficUpdater::new(
          client.clone(),
          prometheus_url.clone(),
          TimeSelection::ThreeMonths,
        )),
        year: Cache::new(TrafficUpdater::new(
          client.clone(),
          prometheus_url.clone(),
          TimeSelection::Year,
        )),
      }),
      as112: Arc::new(TimeSelectionStore {
        two_days: Cache::new(As112Updater::new(
          client.clone(),
          prometheus_url.clone(),
          TimeSelection::TwoDays,
        )),
        week: Cache::new(As112Updater::new(
          client.clone(),
          prometheus_url.clone(),
          TimeSelection::Week,
        )),
        month: Cache::new(As112Updater::new(
          client.clone(),
          prometheus_url.clone(),
          TimeSelection::Month,
        )),
        three_months: Cache::new(As112Updater::new(
          client.clone(),
          prometheus_url.clone(),
          TimeSelection::ThreeMonths,
        )),
        year: Cache::new(As112Updater::new(
          client.clone(),
          prometheus_url.clone(),
          TimeSelection::Year,
        )),
      }),
    }
  }

  pub(crate) async fn update(&self) -> anyhow::Result<()> {
    for selection in TimeSelection::iter() {
      self.traffic.get(selection).update().await?;
      self.as112.get(selection).update().await?;
    }

    Ok(())
  }

  pub(crate) async fn get_traffic_stats(
    &self,
    selection: TimeSelection,
  ) -> Option<Arc<Series<Vec<(f64, f64)>>>> {
    self.traffic.get(selection).get().await
  }

  pub(crate) async fn get_as112_stats(
    &self,
    selection: TimeSelection,
  ) -> Option<Arc<Series<HashMap<String, Vec<(f64, f64)>>>>> {
    self.as112.get(selection).get().await
  }
}
