use std::collections::HashSet;
use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Deserializer};

use crate::blog::{BlogPost, SmallBlogPost};
use crate::lang::Language;
use crate::state::FoundationState;

#[derive(Deserialize)]
pub(crate) struct ListQuery {
  keywords: Option<P>,
}

#[derive(Deserialize)]
struct P(#[serde(deserialize_with = "tags_deserialize")] Vec<String>);

fn tags_deserialize<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
  D: Deserializer<'de>,
{
  let str_sequence = String::deserialize(deserializer)?;
  Ok(
    str_sequence
      .split(',')
      .map(|item| item.to_owned())
      .collect(),
  )
}

pub(crate) async fn list_posts(
  State(state): State<FoundationState>,
  Path(lang): Path<Language>,
  Query(query): Query<ListQuery>,
) -> Json<Vec<Arc<SmallBlogPost>>> {
  match query.keywords {
    None => Json(state.blog.content_by_lang(lang).await),
    Some(keywords) => Json(state.blog.search_by_keywords(lang, &keywords.0).await),
  }
}

pub(crate) async fn find_post(
  State(state): State<FoundationState>,
  Path((lang, slug)): Path<(Language, String)>,
) -> Result<Json<Arc<BlogPost>>, StatusCode> {
  state
    .blog
    .content_by_slug(lang, &slug)
    .await
    .map(Json)
    .ok_or(StatusCode::NOT_FOUND)
}

pub(crate) async fn find_keywords(State(state): State<FoundationState>) -> Json<HashSet<String>> {
  Json(state.blog.keywords().await)
}
