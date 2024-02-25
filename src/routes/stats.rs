use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use tracing::error;

use crate::state::FoundationState;
use crate::stats::AggregatedStats;

pub(super) async fn get_stats(
  State(state): State<FoundationState>,
) -> Result<Json<Arc<AggregatedStats>>, StatusCode> {
  match state.stats.get_stats().await {
    Ok(stats) => Ok(Json(stats)),
    Err(err) => {
      error!("Error while querying stats: {:?}", err);
      Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
  }
}
