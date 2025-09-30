use super::parse_file_name;
use std::collections::HashSet;
use std::path::Path;
use std::sync::Arc;

use asciidork_parser::parser::SourceFile;
use asciidork_parser::prelude::Bump;
use asciidork_parser::Parser;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tracing::info;

use crate::lang::Language;

pub trait LongPostFromMeta<Meta> {
  fn from(slug: &str, lang: Language, idx: u32, meta: Meta, body: String) -> Self;
}

pub trait PostMeta {
  fn idx(&self) -> u32;
  fn lang(&self) -> Language;
  fn slug(&self) -> &str;
  fn keywords(&self) -> &Vec<String>;
}

#[derive(Debug, Clone)]
pub(crate) struct PostProvider<Meta, ShortPost, LongPost>
where
  LongPost: LongPostFromMeta<Meta>,
  ShortPost: From<Arc<LongPost>>,
{
  posts: Arc<Vec<Arc<LongPost>>>,
  small_posts: Arc<Vec<Arc<ShortPost>>>,
  _meta: Arc<Vec<Arc<Meta>>>,
}

impl<Meta, ShortPost, LongPost> PostProvider<Meta, ShortPost, LongPost>
where
  LongPost: Serialize + LongPostFromMeta<Meta> + PostMeta + std::fmt::Debug,
  ShortPost: Serialize + From<Arc<LongPost>> + PostMeta + PartialEq,
  Meta: DeserializeOwned + Clone,
{
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
      let (meta, text) = content.split_once("---").unwrap();

      let meta: Meta = serde_yaml_ng::from_str::<Meta>(meta)?;
      let file_name = path.file_name().unwrap().to_str().unwrap();

      if file_name.starts_with('_') {
        continue;
      }

      let is_adoc_file = file_name.ends_with(".adoc");

      info!(
        "reading news post: {} is adoc: {}",
        &file_name, &is_adoc_file
      );
      let (idx, lang, slug) = parse_file_name(file_name)?;

      let body = if is_adoc_file {
        let bump = &Bump::with_capacity(text.len() * 2);
        let parsed_adoc = Parser::from_str(text, SourceFile::Path(path.clone().into()), bump)
          .parse()
          .expect("cannot parse adoc!");

        asciidork_dr_html_backend::convert(parsed_adoc.document).expect("cannot render adoc!")
      } else {
        markdown::to_html(text)
      };

      posts.push(Arc::new(LongPost::from(slug, lang, idx, meta, body)));
    }

    posts.sort_by_key(|b| std::cmp::Reverse(b.idx()));

    let small_posts = posts
      .iter()
      .map(|post| Arc::new(ShortPost::from(post.clone())))
      .collect();

    Ok(Self {
      posts: Arc::new(posts),
      small_posts: Arc::new(small_posts),
      _meta: Arc::new(Vec::new()),
    })
  }

  pub async fn content_by_lang(&self, lang: Language) -> Vec<Arc<ShortPost>> {
    self
      .small_posts
      .iter()
      .filter(|post| post.lang() == lang)
      .cloned()
      .collect()
  }

  pub async fn content_by_slug(&self, lang: Language, slug: &str) -> Option<Arc<LongPost>> {
    self
      .posts
      .iter()
      .find(|post| post.lang() == lang && post.slug() == slug)
      .cloned()
  }

  pub async fn search_by_keywords(
    &self,
    lang: Language,
    keywords: &[String],
  ) -> Vec<Arc<ShortPost>> {
    let posts = self
      .small_posts
      .iter()
      .filter(|post| post.lang() == lang)
      .collect::<Vec<_>>();

    let keywords_set = keywords.iter().collect::<HashSet<_>>();

    let mut or = posts
      .iter()
      .filter(|post| {
        post
          .keywords()
          .iter()
          .collect::<HashSet<_>>()
          .intersection(&keywords_set)
          .next()
          .is_some()
      })
      .cloned()
      .cloned()
      .collect::<Vec<_>>();

    let mut and = posts
      .iter()
      .filter(|post| {
        !or.contains(post)
          && post
            .keywords()
            .iter()
            .collect::<HashSet<_>>()
            .intersection(&keywords_set)
            .count()
            == keywords.len()
      })
      .cloned()
      .cloned()
      .collect::<Vec<_>>();

    or.append(&mut and);

    or
  }

  pub(crate) async fn keywords(&self) -> HashSet<String> {
    self
      .small_posts
      .iter()
      .flat_map(|post| post.keywords().clone())
      .collect()
  }
}
