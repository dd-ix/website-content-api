use std::sync::Arc;

use tokio::sync::RwLock;

pub(crate) trait Updater {
  type Output;
  type Error;
  async fn update(&self) -> Result<Self::Output, Self::Error>;
}

pub(crate) struct Cache<U: Updater> {
  cached: RwLock<Option<Arc<U::Output>>>,
  updater: U,
}

impl<U: Updater + Sync> Cache<U> {
  pub(crate) fn new(updater: U) -> Self {
    Self {
      cached: RwLock::new(None),
      updater,
    }
  }

  pub(crate) async fn update(&self) -> Result<(), U::Error> {
    let new = Arc::new(self.updater.update().await?);
    *self.cached.write().await = Some(new.clone());
    Ok(())
  }

  pub(crate) async fn get(&self) -> Option<Arc<U::Output>> {
    let lock = self.cached.read().await;
    lock.as_ref().cloned()
  }
}
