use crate::looking_glass::is_address_in_network;
use crate::state::FoundationState;
use axum::extract::{ConnectInfo, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::debug;

#[derive(Serialize)]
pub struct NetworkInformation {
  pub is_connected: bool,
}

pub(crate) async fn get_connected_to_community(
  ConnectInfo(addr): ConnectInfo<SocketAddr>,
  State(state): State<FoundationState>,
) -> Result<Json<Arc<NetworkInformation>>, StatusCode> {
  debug!("request from: {addr}");
  let routes = state.looking_glass.routes.get_cached();

  Ok(Json(Arc::new(NetworkInformation {
    is_connected: is_address_in_network(&(routes.await), addr.ip()),
  })))
}
