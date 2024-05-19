use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use tracing::error;

use crate::state::FoundationState;
use crate::stats::{Series, TimeSelection};

pub(super) async fn get_traffic_stats(
  Path(selection): Path<TimeSelection>,
  State(state): State<FoundationState>,
) -> Result<Json<Arc<Series>>, StatusCode> {
  match state.stats.get_traffic_stats(selection).await {
    Ok(stats) => Ok(Json(stats)),
    Err(err) => {
      error!("Error while querying traffic stats: {:?}", err);
      Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
  }
}

pub(super) async fn get_as112_stats(
  Path(selection): Path<TimeSelection>,
  State(state): State<FoundationState>,
) -> Result<Json<Arc<Series>>, StatusCode> {
  match state.stats.get_as112_stats(selection).await {
    Ok(stats) => Ok(Json(stats)),
    Err(err) => {
      error!("Error while querying as112 stats: {:?}", err);
      Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
  }
}
