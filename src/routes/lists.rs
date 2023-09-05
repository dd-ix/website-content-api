use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use log::error;

use crate::lists::{MailingListsError, Subscriber};
use crate::state::FoundationState;

pub(crate) async fn add_subscriber(
  State(state): State<FoundationState>,
  Path(list): Path<i32>,
  Json(payload): Json<Subscriber>,
) -> StatusCode {
  if !payload.valid_email() {
    return StatusCode::BAD_REQUEST;
  }

  match state
    .lists
    .submit_to_listmonk(payload, list)
    .await
    .map_err(|e| {
      error!("couldn't add subscriber because of {:?}", e);
      e
    }) {
    Ok(_) => StatusCode::OK,
    Err(MailingListsError::InvalidMailingListId) => StatusCode::BAD_REQUEST,
    Err(MailingListsError::ListmonkError) => StatusCode::INTERNAL_SERVER_ERROR,
    Err(MailingListsError::RequestError) => StatusCode::INTERNAL_SERVER_ERROR,
  }
}
