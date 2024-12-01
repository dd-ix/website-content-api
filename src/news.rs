use crate::lang::Language;
use crate::posts::{
  post_provider::{LongPostFromMeta, PostMeta, PostProvider},
  MyDate,
};
use serde::{Deserialize, Serialize};

pub(crate) type News = PostProvider<NewsMeta, SmallNewsPost, NewsPost>;

#[derive(Deserialize, Clone)]
pub(crate) struct NewsMeta {
  title: String,
  published: MyDate,
  modified: Option<MyDate>,
  description: String,
  keywords: Vec<String>,
  authors: Vec<String>,
  image: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
pub(crate) struct NewsPost {
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

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub(crate) struct SmallNewsPost {
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

impl LongPostFromMeta<NewsMeta> for NewsPost {
  fn from(slug: &str, lang: Language, idx: u32, meta: NewsMeta, body: String) -> Self {
    Self {
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
      body,
    }
  }
}

impl From<NewsPost> for SmallNewsPost {
  fn from(post: NewsPost) -> Self {
    Self {
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
    }
  }
}

impl PostMeta for SmallNewsPost {
  fn idx(&self) -> u32 {
    self.idx
  }

  fn lang(&self) -> Language {
    self.lang
  }

  fn slug(&self) -> &str {
    &self.slug
  }

  fn keywords(&self) -> &Vec<String> {
    &self.keywords
  }
}

impl PostMeta for NewsPost {
  fn idx(&self) -> u32 {
    self.idx
  }

  fn lang(&self) -> Language {
    self.lang
  }

  fn slug(&self) -> &str {
    &self.slug
  }

  fn keywords(&self) -> &Vec<String> {
    &self.keywords
  }
}
