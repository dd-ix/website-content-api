use std::path::PathBuf;

use crate::routes::bird::get_bird;
use axum::routing::{get, post};
use axum::Router;
use tower_http::services::ServeDir;

use crate::routes::documents::list_documents;
use crate::routes::event::{find_event, list_all_events, list_future_events};
use crate::routes::lists::add_subscriber;
use crate::routes::blog::{find_keywords, find_post, list_posts};
use crate::routes::peers::get_peers_and_supporter;
use crate::routes::team::get_team;
use crate::routes::text_blocks::find_text_block;
use crate::state::FoundationState;

use self::stats::{get_as112_stats, get_traffic_stats};

mod bird;
mod documents;
mod event;
mod lists;
mod blog;
mod peers;
mod stats;
mod team;
mod text_blocks;

pub(crate) struct ContentPaths {
  pub(crate) blog: PathBuf,
  pub(crate) event: PathBuf,
  pub(crate) text_blocks: PathBuf,
  pub(crate) document: PathBuf,
  pub(crate) team: PathBuf,
}

pub(crate) fn route(content_paths: &ContentPaths) -> Router<FoundationState> {
  Router::new()
    .route("/blog/:lang", get(list_posts))
    .route("/blog/:lang/:slug", get(find_post))
    .route("/blog/keywords", get(find_keywords))
    .route("/event/:lang/all", get(list_all_events))
    .route("/event/:lang/upcoming", get(list_future_events))
    .route("/event/:lang/:slug", get(find_event))
    .route("/text-blocks/:lang/:slug", get(find_text_block))
    .nest_service(
      "/text-blocks/assets",
      ServeDir::new(&content_paths.text_blocks),
    )
    .nest_service("/blog/assets", ServeDir::new(&content_paths.blog))
    .nest_service("/event/assets", ServeDir::new(&content_paths.event))
    .route("/documents/:lang", get(list_documents))
    .nest_service(
      "/documents/download",
      ServeDir::new(&content_paths.document),
    )
    .route("/team/:lang", get(get_team))
    .nest_service("/team/assets", ServeDir::new(&content_paths.team))
    .route("/mailing_lists/:list", post(add_subscriber))
    .route("/stats/traffic/:selection", get(get_traffic_stats))
    .route("/stats/as112/:selection", get(get_as112_stats))
    .route("/peers", get(get_peers_and_supporter))
    .route("/bird", get(get_bird))
}
