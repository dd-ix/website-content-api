use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Mirror {
  pub(crate) name: String,
  pub(crate) url: String,
  pub(crate) operator: String,
}

#[derive(Clone)]
pub(crate) struct Mirrors {
  mirrors: Vec<Mirror>,
}

impl Mirrors {
  pub(crate) async fn load(file: impl AsRef<Path>) -> anyhow::Result<Self> {
    let serialized_mirrors = tokio::fs::read_to_string(file).await?;
    let mut mirrors: Vec<Mirror> = serde_yaml_ng::from_str(&serialized_mirrors)?;

    mirrors.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(Self { mirrors })
  }

  pub(crate) fn mirrors(&self) -> Vec<Mirror> {
    self.mirrors.to_vec()
  }
}
