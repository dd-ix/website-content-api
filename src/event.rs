use std::sync::Arc;

use crate::lang::Language;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use url::Url;

use crate::posts::post_provider::{LongPostFromMeta, PostMeta, PostProvider};

pub(crate) type Events = PostProvider<EventMeta, SmallEventPost, EventPost>;

#[derive(Deserialize, Clone)]
pub(crate) struct EventMeta {
  title: String,
  #[serde(deserialize_with = "time::serde::iso8601::deserialize")]
  start_time: OffsetDateTime,
  #[serde(deserialize_with = "time::serde::iso8601::deserialize")]
  end_time: OffsetDateTime,
  location: String,
  description: String,
  keywords: Vec<String>,
  image: Option<String>,
  link: Option<Url>,
}

#[derive(Serialize, Debug, Clone)]
pub(crate) struct EventPost {
  slug: String,
  lang: Language,
  idx: u32,
  title: String,
  #[serde(serialize_with = "time::serde::iso8601::serialize")]
  start_time: OffsetDateTime,
  #[serde(serialize_with = "time::serde::iso8601::serialize")]
  end_time: OffsetDateTime,
  location: String,
  description: String,
  keywords: Vec<String>,
  image: Option<String>,
  link: Option<Url>,
  body: String,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub(crate) struct SmallEventPost {
  slug: String,
  lang: Language,
  idx: u32,
  title: String,
  #[serde(serialize_with = "time::serde::iso8601::serialize")]
  pub start_time: OffsetDateTime,
  #[serde(serialize_with = "time::serde::iso8601::serialize")]
  pub end_time: OffsetDateTime,
  location: String,
  description: String,
  keywords: Vec<String>,
  image: Option<String>,
}

impl LongPostFromMeta<EventMeta> for EventPost {
  fn from(slug: &str, lang: Language, idx: u32, meta: EventMeta, body: String) -> Self {
    Self {
      slug: slug.to_string(),
      lang,
      idx,
      title: meta.title,
      start_time: meta.start_time,
      end_time: meta.end_time,
      location: meta.location,
      description: meta.description,
      keywords: meta.keywords,
      image: meta.image,
      link: meta.link,
      body,
    }
  }
}

impl From<Arc<EventPost>> for SmallEventPost {
  fn from(event: Arc<EventPost>) -> Self {
    Self {
      slug: event.slug.clone(),
      lang: event.lang,
      idx: event.idx,
      title: event.title.clone(),
      start_time: event.start_time,
      end_time: event.end_time,
      location: event.location.clone(),
      description: event.description.clone(),
      keywords: event.keywords.clone(),
      image: event.image.clone(),
    }
  }
}

impl PostMeta for SmallEventPost {
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

impl PostMeta for EventPost {
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
