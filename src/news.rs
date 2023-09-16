use std::path::Path;
use std::sync::Arc;

use anyhow::anyhow;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashSet;
use time::Date;

use crate::lang::Language;

#[derive(Copy, Clone, Debug)]
struct MyDate(Date);

#[derive(Debug, Clone)]
pub(crate) struct News {
  posts: Arc<Vec<Arc<Post>>>,
  small_posts: Arc<Vec<Arc<SmallPost>>>,
}

#[derive(Deserialize)]
pub(crate) struct WrittenPostMeta {
  title: String,
  published: MyDate,
  modified: Option<MyDate>,
  description: String,
  keywords: Vec<String>,
  authors: Vec<String>,
  image: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
pub(crate) struct Post {
  slug: String,
  lang: Language,
  idx: u32,
  title: String,
  published: MyDate,
  modified: Option<MyDate>,
  description: String,
  keywords: Vec<String>,
  authors: Vec<String>,
  image: Option<String>,
  body: String,
}

#[derive(Serialize, Debug, Clone)]
pub(crate) struct SmallPost {
  slug: String,
  lang: Language,
  idx: u32,
  title: String,
  published: MyDate,
  modified: Option<MyDate>,
  description: String,
  keywords: Vec<String>,
  authors: Vec<String>,
  image: Option<String>,
}

impl News {
  pub(crate) async fn load(directory: &Path) -> anyhow::Result<Self> {
    let mut posts = Vec::new();

    let mut dir = tokio::fs::read_dir(directory).await?;
    while let Some(entry) = dir.next_entry().await? {
      if entry.file_type().await?.is_dir() {
        continue;
      }

      let path = entry.path();
      let content = tokio::fs::read_to_string(path.as_path()).await?;
      let content = content.trim_start();
      let content = content.strip_prefix("---").unwrap();
      let (meta, body) = content.split_once("---").unwrap();

      let meta: WrittenPostMeta = serde_yaml::from_str(meta)?;
      let file_name = path.file_name().unwrap().to_str().unwrap();
      let (idx, lang, slug) = parse_file_name(file_name)?;

      posts.push(Arc::new(Post {
        slug: slug.to_string(),
        lang,
        idx,
        title: meta.title,
        published: meta.published,
        modified: meta.modified,
        description: meta.description,
        keywords: meta.keywords,
        authors: meta.authors,
        image: meta.image,
        body: markdown::to_html(body),
      }));
    }

    posts.sort_by(|a, b| a.idx.cmp(&b.idx));

    let small_posts = posts
      .iter()
      .map(|post| {
        Arc::new(SmallPost {
          slug: post.slug.clone(),
          lang: post.lang,
          idx: post.idx,
          title: post.title.clone(),
          published: post.published,
          modified: post.modified,
          description: post.description.clone(),
          keywords: post.keywords.clone(),
          authors: post.authors.clone(),
          image: post.image.clone(),
        })
      })
      .collect();

    Ok(News {
      posts: Arc::new(posts),
      small_posts: Arc::new(small_posts),
    })
  }

  pub(crate) fn posts(&self, lang: Language) -> Vec<Arc<SmallPost>> {
    self
      .small_posts
      .iter()
      .filter(|post| post.lang == lang)
      .cloned()
      .collect()
  }

  pub(crate) fn find_post(&self, lang: Language, slug: &str) -> Option<Arc<Post>> {
    self
      .posts
      .iter()
      .find(|post| post.lang == lang && post.slug == slug)
      .cloned()
  }

  pub(crate) fn search_by_keywords(
    &self,
    lang: Language,
    keywords: &Vec<String>,
  ) -> Vec<Arc<SmallPost>> {
    self
      .small_posts
      .iter()
      .filter(|post| post.lang == lang)
      .filter(|post| {
        let unique_a = post.keywords.iter().collect::<HashSet<_>>();
        let unique_b = keywords.iter().collect::<HashSet<_>>();

        let result = unique_a.intersection(&unique_b).collect::<Vec<_>>();
        !result.is_empty()
      })
      .cloned()
      .collect()
  }

  pub(crate) fn keywords(&self) -> HashSet<String> {
    self
      .small_posts
      .iter()
      .flat_map(|post| post.keywords.clone())
      .collect()
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

impl Serialize for MyDate {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let s = format!(
      "{:0>4}-{:0>2}-{:0>2}",
      self.0.year(),
      self.0.month() as u8,
      self.0.day()
    );

    serializer.serialize_str(&s)
  }
}

impl<'de> Deserialize<'de> for MyDate {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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
    .map(MyDate)
  }
}
