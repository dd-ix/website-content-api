use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;
use axum::Json;
use tracing::error;

use crate::state::FoundationState;

#[axum::debug_handler]
pub(super) async fn get_bird(
  State(state): State<FoundationState>,
) -> Result<Html<String>, StatusCode> {
  match state.bird.content().await {
    Ok(bird) => Ok(Html(bird.to_string())),
    Err(err) => {
      error!("Error while querying bird: {:?}", err);
      Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
  }
}
