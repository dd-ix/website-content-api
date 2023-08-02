use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::lang::Language;

#[derive(Serialize, Deserialize)]
pub(crate) struct Document {
  name: String,
  description: String,
  filename: String,
}

#[derive(Clone)]
pub(crate) struct Documents {
  documents: Arc<HashMap<Language, Arc<Vec<Document>>>>,
}

impl Documents {
  pub(crate) async fn load(path: &Path) -> anyhow::Result<Self> {
    let string = tokio::fs::read_to_string(path.join("documents.yaml")).await?;
    let documents = serde_yaml::from_str(&string)?;
    Ok(Self {
      documents,
    })
  }

  pub(crate) fn documents(&self, lang: Language) -> Option<Arc<Vec<Document>>> {
    self.documents.get(&lang).cloned()
  }
}
