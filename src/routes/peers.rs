use crate::peers::FoundationEntity;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use tracing::{error, info};

use crate::state::FoundationState;

pub(super) async fn get_peers_and_supporter(
  State(state): State<FoundationState>,
) -> Result<Json<Vec<FoundationEntity>>, StatusCode> {
  info!("requested peers list!");
  match state.peers.get_stats().await {
    Ok(stats) => Ok(Json(stats)),
    Err(err) => {
      error!("Error while querying peers: {:?}", err);
      Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
  }
}
