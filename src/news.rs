use std::path::Path;
use std::sync::Arc;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use time::Date;

use crate::lang::Language;

#[derive(Debug, Clone)]
pub(crate) struct News {
  posts: Arc<Vec<Post>>,
  small_posts: Arc<Vec<SmallPost>>,
}

#[derive(Deserialize)]
pub(crate) struct WrittenPostMeta {
  title: String,
  #[serde(with = "my_date_format")]
  published: Date,
  #[serde(with = "my_date_format")]
  modified: Date,
  description: String,
  keywords: Vec<String>,
  authors: Vec<String>,
}

#[derive(Serialize, Debug, Clone)]
pub(crate) struct Post {
  slug: String,
  lang: Language,
  idx: u32,
  title: String,
  #[serde(with = "my_date_format")]
  published: Date,
  #[serde(with = "my_date_format")]
  modified: Date,
  description: String,
  keywords: Vec<String>,
  authors: Vec<String>,
  body: String,
}

#[derive(Serialize, Debug, Clone)]
pub(crate) struct SmallPost {
  slug: String,
  lang: Language,
  idx: u32,
  title: String,
  #[serde(with = "my_date_format")]
  published: Date,
  #[serde(with = "my_date_format")]
  modified: Date,
  description: String,
  keywords: Vec<String>,
  authors: Vec<String>,
}

impl News {
  pub(crate) async fn load(directory: &Path) -> anyhow::Result<Self> {
    let mut posts = Vec::new();

    let mut dir = tokio::fs::read_dir(directory).await?;
    while let Some(entry) = dir.next_entry().await? {
      let path = entry.path();
      let content = tokio::fs::read_to_string(path.as_path()).await?;
      let content = content.trim_start();
      let content = content.strip_prefix("---").unwrap();
      let (meta, body) = content.split_once("---").unwrap();

      let meta: WrittenPostMeta = serde_yaml::from_str(meta)?;
      let file_name = path.file_name().unwrap().to_str().unwrap();
      let (idx, lang, slug) = parse_file_name(file_name)?;

      posts.push(Post {
        slug: slug.to_string(),
        lang,
        idx,
        title: meta.title,
        published: meta.published,
        modified: meta.modified,
        description: meta.description,
        keywords: meta.keywords,
        authors: meta.authors,
        body: markdown::to_html(body),
      });
    }

    posts.sort_by(|a, b| a.idx.cmp(&b.idx));

    let small_posts = posts
      .iter()
      .map(|post| SmallPost {
        slug: post.slug.clone(),
        lang: post.lang,
        idx: post.idx,
        title: post.title.clone(),
        published: post.published,
        modified: post.modified,
        description: post.description.clone(),
        keywords: post.keywords.clone(),
        authors: post.authors.clone(),
      })
      .collect();

    Ok(News {
      posts: Arc::new(posts),
      small_posts: Arc::new(small_posts),
    })
  }

  pub(crate) fn posts(&self) -> &[SmallPost] {
    &self.small_posts
  }

  pub(crate) fn find_post(&self, lang: Language, slug: &str) -> Option<&Post> {
    self
      .posts
      .iter()
      .find(|post| post.lang == lang && post.slug == slug)
  }
}

fn parse_file_name(file_name: &str) -> anyhow::Result<(u32, Language, &str)> {
  let mut split = file_name.split('.');

  let idx = split
    .next()
    .ok_or_else(|| anyhow!("Index missing in file name {}", file_name))?
    .parse()?;
  let slug = split
    .next()
    .ok_or_else(|| anyhow!("Slug missing in file name {}", file_name))?;
  let lang = split
    .next()
    .ok_or_else(|| anyhow!("Language missing in file name {}", file_name))?
    .try_into()?;

  Ok((idx, lang, slug))
}

mod my_date_format {
  use serde::de::Error;
  use serde::{self, Deserialize, Deserializer, Serializer};
  use time::Date;

  pub fn serialize<S>(date: &Date, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let s = format!(
      "{:0>4}-{:0>2}-{:0>2}",
      date.year(),
      date.month() as u8,
      date.day()
    );
    serializer.serialize_str(&s)
  }

  // The signature of a deserialize_with function must follow the pattern:
  //
  //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
  //    where
  //        D: Deserializer<'de>
  //
  // although it may also be generic over the output types T.
  pub fn deserialize<'de, D>(deserializer: D) -> Result<Date, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    let mut split = s.split('-');

    let year = split
      .next()
      .ok_or_else(|| Error::custom(format!("Invalid date format {}", s)))?
      .parse()
      .map_err(|e| Error::custom(format!("{}", e)))?;

    let month: u8 = split
      .next()
      .ok_or_else(|| Error::custom(format!("Invalid date format {}", s)))?
      .parse()
      .map_err(|e| Error::custom(format!("{}", e)))?;

    let day = split
      .next()
      .ok_or_else(|| Error::custom(format!("Invalid date format {}", s)))?
      .parse()
      .map_err(|e| Error::custom(format!("{}", e)))?;

    Date::from_calendar_date(
      year,
      month
        .try_into()
        .map_err(|e| Error::custom(format!("{}", e)))?,
      day,
    )
    .map_err(|e| Error::custom(format!("{}", e)))
  }
}
