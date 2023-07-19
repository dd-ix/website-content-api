use axum::routing::get;
use axum::Router;

use crate::routes::documents::list_documents;
use crate::routes::static_content::download_static_content;
use crate::routes::news::{find_post, list_posts};
use crate::state::FoundationState;

mod documents;
mod static_content;
mod news;

pub(crate) fn route() -> Router<FoundationState> {
  Router::new()
    .route("/news/:lang", get(list_posts))
    .route("/news/:lang/:slug", get(find_post))
    .route("/documents/:lang", get(list_documents))
    .route("/static/:file", get(download_static_content))
}
