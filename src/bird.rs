use axum::async_trait;
use select::predicate::Name;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::cache::{Cache, Updater};

struct BirdUpdater {
  path: PathBuf,
}

#[derive(Clone)]
pub(crate) struct Bird {
  content: Arc<Cache<BirdUpdater>>,
}

#[async_trait]
impl Updater for BirdUpdater {
  type Output = String;
  type Error = anyhow::Error;

  async fn update(&self) -> Result<Self::Output, Self::Error> {
    let content = load(&self.path).await?;
    Ok(content)
  }
}

impl Bird {
  pub(crate) async fn new(path: PathBuf) -> anyhow::Result<Self> {
    Ok(Self {
      content: Arc::new(Cache::new(BirdUpdater { path })),
    })
  }

  pub(crate) async fn content(&self) -> anyhow::Result<Arc<String>> {
    self.content.get().await
  }
}

async fn load(path: &Path) -> anyhow::Result<String> {
  let content = select::document::Document::from(tokio::fs::read_to_string(path).await?.as_str());
  Ok(content.find(Name("body")).next().unwrap().inner_html())
}
