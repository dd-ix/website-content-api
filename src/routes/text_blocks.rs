use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;

use crate::lang::Language;
use crate::state::FoundationState;
use crate::text_blocks::TextBlock;

pub(crate) async fn find_text_block(
  State(state): State<FoundationState>,
  Path((lang, slug)): Path<(Language, String)>,
) -> Result<Json<Arc<TextBlock>>, StatusCode> {
  state
    .text_blocks
    .find_text_block(lang, &slug)
    .map(Json)
    .ok_or(StatusCode::NOT_FOUND)
}
