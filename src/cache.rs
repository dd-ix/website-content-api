use std::sync::{
  atomic::{AtomicBool, Ordering},
  Arc,
};

use time::{Duration, OffsetDateTime};
use tokio::sync::RwLock;

const UPDATING_WAIT: std::time::Duration = std::time::Duration::from_millis(10);
const MAX_AGE: Duration = Duration::minutes(10);

pub(crate) trait Updater {
  type Output;
  type Error;
  async fn update(&self) -> Result<Self::Output, Self::Error>;
}

pub(crate) struct Cache<U: Updater> {
  updating: AtomicBool,
  cached: RwLock<Option<(OffsetDateTime, Arc<U::Output>)>>,
  updater: U,
}

impl<U: Updater> Cache<U> {
  pub(crate) fn new(updater: U) -> Self {
    Self {
      updating: AtomicBool::new(false),
      cached: RwLock::new(None),
      updater,
    }
  }

  pub(crate) async fn get(&self) -> Result<Arc<U::Output>, U::Error> {
    let now = OffsetDateTime::now_utc();

    {
      while self.updating.load(Ordering::Relaxed) {
        tokio::time::sleep(UPDATING_WAIT).await
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

    let new = match self.updater.update().await {
      Ok(new) => {
        let new = Arc::new(new);
        *self.cached.write().await = Some((now + MAX_AGE, new.clone()));
        Ok(new)
      }
      Err(err) => Err(err),
    };

    self.updating.store(false, Ordering::Relaxed);

    new
  }
}
