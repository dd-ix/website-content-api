use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

pub(crate) struct News {
    directory: PathBuf,
}

#[derive(Deserialize)]
pub(crate) struct WrittenPostMeta {
    name: String,
    published: OffsetDateTime,
    modified: OffsetDateTime,
    description: String,
    keywords: Vec<String>,
    authors: Vec<String>,
}

#[derive(Serialize)]
pub(crate) struct PostMeta {
    slug: String,
    name: String,
    published: OffsetDateTime,
    modified: OffsetDateTime,
    description: String,
    keywords: Vec<String>,
    authors: Vec<String>,
}

impl News {
    pub(crate) async fn list_posts(&self) -> anyhow::Result<Vec<PostMeta>> {
        let mut posts = Vec::new();

        let mut dir = tokio::fs::read_dir(&self.directory).await?;
        while let Some(entry) = dir.next_entry().await? {
            let path = entry.path();
            let content = tokio::fs::read_to_string(path.as_path()).await?;
            let content = content.trim_start();
            let content = content.strip_prefix("---").unwrap();
            let (meta, _) = content.split_once("---").unwrap();

            let meta: WrittenPostMeta = serde_yaml::from_str(meta)?;
            let slug = entry.path().file_stem().and_then(|x| x.to_str()).unwrap().to_string();

            posts.push(PostMeta {
                slug,
                name: meta.name,
                published: meta.published,
                modified: meta.modified,
                description: meta.description,
                keywords: meta.keywords,
                authors: meta.authors,
            });
        }

        Ok(posts)
    }
}