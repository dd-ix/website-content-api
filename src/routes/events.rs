use std::sync::Arc;

use axum::extract::{Path, State};
use axum::Json;
use crate::events::Event;

use crate::lang::Language;
use crate::state::FoundationState;

pub(crate) async fn list_all_events(
  State(state): State<FoundationState>,
  Path(lang): Path<Language>,
) -> Json<Vec<Arc<Event>>> {
    Json(state.events.get_all_events(&lang).await)
}

pub(crate) async fn list_future_events(
  State(state): State<FoundationState>,
  Path(lang): Path<Language>,
) -> Json<Vec<Arc<Event>>> {
  Json(state.events.get_future_events(&lang).await)
}
