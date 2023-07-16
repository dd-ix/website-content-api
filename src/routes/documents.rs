use std::sync::Arc;

use axum::extract::{Path, State};
use axum::Json;

use crate::documents::Document;
use crate::lang::Language;
use crate::state::FoundationState;

pub(crate) async fn list_documents(
  State(state): State<FoundationState>,
  Path(lang): Path<Language>,
) -> Json<Arc<Vec<Document>>> {
  Json(state.documents.documents(lang).unwrap())
}
