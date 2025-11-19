use crate::looking_glass::is_address_in_network;
use crate::state::FoundationState;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use serde::Serialize;
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::Arc;
use tracing::info;

#[derive(Serialize)]
pub struct NetworkInformation {
  pub is_connected: bool,
}

pub(crate) async fn get_connected_to_community(
  headers: HeaderMap,
  State(state): State<FoundationState>,
) -> Result<Json<Arc<NetworkInformation>>, StatusCode> {
  let header_value = match headers.get("X-Forwarded-For") {
    Some(header_value) => header_value,
    None => return Err(StatusCode::BAD_REQUEST),
  };
  let addr = header_value
    .to_str()
    .map_err(|_| StatusCode::BAD_REQUEST)
    .and_then(|string_value| IpAddr::from_str(string_value).map_err(|_| StatusCode::BAD_REQUEST))?;

  info!("addr is {}", addr);
  let routes = match state.looking_glass.routes.get_cached().await {
    Some(routes) => routes,
    None => return Err(StatusCode::SERVICE_UNAVAILABLE),
  };

  Ok(Json(Arc::new(NetworkInformation {
    is_connected: is_address_in_network(&(routes), addr),
  })))
}
