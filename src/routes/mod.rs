use std::path::PathBuf;
use axum::routing::get;
use axum::Router;
use tower_http::services::ServeDir;

use crate::routes::documents::list_documents;
use crate::routes::news::{find_post, list_posts};
use crate::state::FoundationState;

mod documents;
mod news;

pub(crate) fn route(static_content_path: &PathBuf) -> Router<FoundationState> {
  Router::new()
    .route("/news/:lang", get(list_posts))
    .route("/news/:lang/:slug", get(find_post))
    .route("/documents/:lang", get(list_documents))
    .nest_service("/static",
                  ServeDir::new(static_content_path),
    )
}
