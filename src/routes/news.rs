use crate::lang::Language;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;

use crate::news::{Post, SmallPost};
use crate::state::FoundationState;

pub(crate) async fn list_posts(State(state): State<FoundationState>) -> Json<Vec<SmallPost>> {
  Json(state.news.posts().to_vec())
}

pub(crate) async fn find_post(
  State(state): State<FoundationState>,
  Path((lang, slug)): Path<(Language, String)>,
) -> Result<Json<Post>, StatusCode> {
  state
    .news
    .find_post(lang, &slug)
    .map(|post| Json(post.clone()))
    .ok_or(StatusCode::NOT_FOUND)
}
