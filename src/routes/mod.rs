use axum::routing::get;
use axum::Router;
use std::path::PathBuf;
use tower_http::services::ServeDir;

use crate::routes::documents::list_documents;
use crate::routes::news::{find_post, list_posts};
use crate::routes::text_blocks::find_text_block;
use crate::state::FoundationState;

mod documents;
mod news;
mod text_blocks;

pub(crate) fn route(
  news_content_path: &PathBuf,
  text_blocks_content_path: &PathBuf,
  document_content_path: &PathBuf,
) -> Router<FoundationState> {
  Router::new()
    .route("/news/:lang", get(list_posts))
    .route("/news/:lang/:slug", get(find_post))
    .route("/text-blocks/:lang/:slug", get(find_text_block))
    .nest_service(
      "/text-blocks/assets",
      ServeDir::new(text_blocks_content_path),
    )
    .nest_service("/news/assets", ServeDir::new(news_content_path))
    .route("/documents/:lang", get(list_documents))
    .nest_service("/documents/download", ServeDir::new(document_content_path))
}
