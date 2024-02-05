use axum::http::header::CONTENT_TYPE;
use axum::http::Method;
use clap::Parser;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::args::Args;
use crate::documents::Documents;
use crate::lists::MailingLists;
use crate::news::News;
use crate::routes::{route, ContentPaths};
use crate::state::FoundationState;
use crate::team::Team;
use crate::text_blocks::TextBlocks;

mod args;
mod documents;
mod lang;
mod lists;
mod news;
mod routes;
mod state;
mod team;
mod text_blocks;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let args = Args::parse();

  let subscriber = FmtSubscriber::builder()
    .with_max_level(Level::INFO)
    .compact()
    .finish();

  tracing::subscriber::set_global_default(subscriber)?;

  info!(concat!(
    "Booting ",
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    "..."
  ));

  let state = FoundationState {
    news: News::load(&args.content_directory.join("news")).await?,
    text_blocks: TextBlocks::load(&args.content_directory.join("text_blocks"), &args.base_url)
      .await?,
    documents: Documents::load(&args.content_directory.join("documents")).await?,
    team: Team::load(&args.content_directory.join("team")).await?,
    lists: MailingLists::load(
      &args.listmonk_url,
      &args.listmonk_user,
      &args.listmonk_password_file,
      &args.listmonk_lists,
    )
    .await?,
  };

  let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
    .allow_headers([CONTENT_TYPE])
    .allow_origin(Any);

  let router = route(&ContentPaths {
    news: args.content_directory.join("news/assets"),
    text_blocks: args.content_directory.join("text_blocks/assets"),
    document: args.content_directory.join("documents/download"),
    team: args.content_directory.join("team/assets"),
  })
  .layer(cors)
  .with_state(state);

  let listener = TcpListener::bind(&args.listen_addr).await?;
  info!("Listening on http://{}...", args.listen_addr);

  let server = axum::serve(listener, router.into_make_service());

  if let Err(err) = server.await {
    error!("Error while serving api: {}", err);
  }

  Ok(())
}
