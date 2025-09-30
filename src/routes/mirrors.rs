use axum::{extract::State, Json};

use crate::{mirrors::Mirror, state::FoundationState};

pub(crate) async fn get_mirrors(State(state): State<FoundationState>) -> Json<Vec<Mirror>> {
  Json(state.mirrors.mirrors())
}
