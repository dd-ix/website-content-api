use std::sync::Arc;

use crate::event::{EventPost, SmallEventPost};
use crate::posts::post_provider::PostMeta;
use axum::extract::{Path, State};
use axum::Json;
use time::OffsetDateTime;

use crate::lang::Language;
use crate::state::FoundationState;

pub(crate) async fn list_all_events(
  State(state): State<FoundationState>,
  Path(lang): Path<Language>,
) -> Json<Vec<Arc<SmallEventPost>>> {
  Json(state.events.content_by_lang(lang).await)
}

pub(crate) async fn list_future_events(
  State(state): State<FoundationState>,
  Path(lang): Path<Language>,
) -> Json<Vec<Arc<SmallEventPost>>> {
  let events: Vec<Arc<SmallEventPost>> = state.events.content_by_lang(lang).await;
  let current_date_time = OffsetDateTime::now_utc();

  Json(
    events
      .iter()
      .filter(|post| post.end_time > current_date_time && post.lang() == lang)
      .cloned()
      .collect(),
  )
}

pub(crate) async fn find_event(
  State(state): State<FoundationState>,
  Path((lang, slug)): Path<(Language, String)>,
) -> Json<Option<Arc<EventPost>>> {
  Json(state.events.content_by_slug(lang, &slug).await)
}
