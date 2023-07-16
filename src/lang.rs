use anyhow::anyhow;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Language {
  #[serde(rename = "en")]
  English,
  #[serde(rename = "de")]
  German,
}

impl TryFrom<&str> for Language {
  type Error = anyhow::Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "en" => Ok(Language::English),
      "de" => Ok(Language::German),
      _ => Err(anyhow!("Invalid lang code {}", value)),
    }
  }
}
