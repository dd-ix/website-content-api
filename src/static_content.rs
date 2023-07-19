use std::path::PathBuf;
use tokio::fs::File;

#[derive(Clone)]
pub struct StaticContent {
  base_path: PathBuf,
}

impl StaticContent {
  pub fn load(path: &PathBuf) -> Self {
    Self {
      base_path: path.into(),
    }
  }

  pub async fn get_file(self, file_name: String) -> anyhow::Result<File> {
    Ok(File::open(self.base_path.join(file_name)).await?)
  }
}
