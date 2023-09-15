use crate::lang::Language;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use std::sync::Arc;

use crate::news::{Post, SmallPost};
use crate::state::FoundationState;

pub(crate) async fn list_posts(
  State(state): State<FoundationState>,
  Path(lang): Path<Language>,
) -> Json<Vec<Arc<SmallPost>>> {
  Json(state.news.posts(lang))
}

pub(crate) async fn find_post(
  State(state): State<FoundationState>,
  Path((lang, slug)): Path<(Language, String)>,
) -> Result<Json<Arc<Post>>, StatusCode> {
  state
    .news
    .find_post(lang, &slug)
    .map(Json)
    .ok_or(StatusCode::NOT_FOUND)
}

pub(crate) async fn search_by_keywords(
  State(state): State<FoundationState>,
  Path(lang): Path<Language>,
  Json(keywords): Json<Vec<String>>,
) -> Json<Vec<Arc<SmallPost>>> {
  Json(state.news.search_by_keywords(lang, &keywords))
}
