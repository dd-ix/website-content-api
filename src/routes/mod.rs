use axum::routing::get;
use axum::Router;

use crate::routes::news::{find_post, list_posts};
use crate::state::MetaState;

mod news;

pub(crate) fn route() -> Router<MetaState> {
  Router::new()
    .route("/news", get(list_posts))
    .route("/news/:language/:slug", get(find_post))
}
