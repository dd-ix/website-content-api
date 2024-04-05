use std::path::{Path, PathBuf};
use std::sync::{Arc};

use select::predicate::Name;
use time::{Duration, OffsetDateTime};
use tokio::sync::{RwLock, Mutex};

const MAX_AGE: Duration = Duration::minutes(10);

#[derive(Clone)]
pub(crate) struct Bird {
  path: PathBuf,
  next_update: Arc<Mutex<OffsetDateTime>>,
  content: Arc<RwLock<Arc<String>>>
}

impl Bird {
  pub(crate) async fn new(path: PathBuf) -> anyhow::Result<Self> {
    let content = load(&path).await?;
    Ok(Self { path, next_update: Arc::new(Mutex::new(OffsetDateTime::now_utc())), content: Arc::new(RwLock::new(Arc::new(content))) })
  }

  pub(crate) async fn content(&self) -> anyhow::Result<Arc<String>> {
    {
      let mut lock = self.next_update.lock().await;
      if  OffsetDateTime::now_utc()>*lock {
        *self.content.write().await = Arc::new(load(&self.path).await?);
        *lock = OffsetDateTime::now_utc();
      }
    }

    Ok(self.content.read().await.clone())
  }
}

async fn load(path: &Path) -> anyhow::Result<String> {
  let content = select::document::Document::from(tokio::fs::read_to_string(path).await?.as_str());
  Ok(content.find(Name("body")).next().unwrap().inner_html())
}
