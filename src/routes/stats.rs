use std::collections::HashMap;
use std::sync::Arc;

use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::{Response, StatusCode};
use axum::Json;
use reqwest::header::RETRY_AFTER;

use crate::state::FoundationState;
use crate::stats::{Series, TimeSelection};

pub(super) async fn get_traffic_stats(
  Path(selection): Path<TimeSelection>,
  State(state): State<FoundationState>,
) -> Result<Json<Arc<Series<Vec<(f64, f64)>>>>, Response<Body>> {
  match state.stats.get_traffic_stats(selection).await {
    Some(stats) => Ok(Json(stats)),
    None => Err(
      Response::builder()
        .status(StatusCode::SERVICE_UNAVAILABLE)
        .header(RETRY_AFTER, 5)
        .body("Try again later, still building cache...".into())
        .unwrap(),
    ),
  }
}

pub(super) async fn get_as112_stats(
  Path(selection): Path<TimeSelection>,
  State(state): State<FoundationState>,
) -> Result<Json<Arc<Series<HashMap<String, Vec<(f64, f64)>>>>>, Response<Body>> {
  match state.stats.get_as112_stats(selection).await {
    Some(stats) => Ok(Json(stats)),
    None => Err(
      Response::builder()
        .status(StatusCode::SERVICE_UNAVAILABLE)
        .header(RETRY_AFTER, 5)
        .body("Try again later, still building cache...".into())
        .unwrap(),
    ),
  }
}
