use axum::routing::get;
use axum::Router;
use std::path::PathBuf;
use tower_http::services::ServeDir;

use crate::routes::documents::list_documents;
use crate::routes::lists::add_subscriber;
use crate::routes::news::{find_post, list_posts};
use crate::routes::team::get_team;
use crate::routes::text_blocks::find_text_block;
use crate::state::FoundationState;

mod documents;
mod lists;
mod news;
mod team;
mod text_blocks;

pub(crate) struct ContentPaths {
  pub(crate) news: PathBuf,
  pub(crate) text_blocks: PathBuf,
  pub(crate) document: PathBuf,
  pub(crate) team: PathBuf,
}

pub(crate) fn route(content_paths: &ContentPaths) -> Router<FoundationState> {
  Router::new()
    .route("/news/:lang", get(list_posts))
    .route("/news/:lang/:slug", get(find_post))
    .route("/text-blocks/:lang/:slug", get(find_text_block))
    .nest_service(
      "/text-blocks/assets",
      ServeDir::new(&content_paths.text_blocks),
    )
    .nest_service("/news/assets", ServeDir::new(&content_paths.news))
    .route("/documents/:lang", get(list_documents))
    .nest_service(
      "/documents/download",
      ServeDir::new(&content_paths.document),
    )
    .route("/team/:lang", get(get_team))
    .nest_service("/team/assets", ServeDir::new(&content_paths.team))
    .route("/mailing_lists/:list", get(add_subscriber))
}
