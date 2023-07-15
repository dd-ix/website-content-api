use axum::routing::get;
use axum::Router;

use crate::routes::news::{find_post, list_posts};
use crate::state::FoundationState;

mod news;

pub(crate) fn route() -> Router<FoundationState> {
  Router::new()
    .route("/news", get(list_posts))
    .route("/news/:lang/:slug", get(find_post))
}
