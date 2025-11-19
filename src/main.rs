use axum::http::header::CONTENT_TYPE;
use axum::http::Method;
use clap::Parser;
use std::time::Duration;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::args::Args;
use crate::bird::Bird;
use crate::blog::Blogs;
use crate::documents::Documents;
use crate::event::Events;
use crate::looking_glass::LookingGlass;
use crate::mirrors::Mirrors;
use crate::news::News;
use crate::peers::NetworkService;
use crate::routes::{route, ContentPaths};
use crate::state::FoundationState;
use crate::stats::Stats;
use crate::team::Team;
use crate::text_blocks::TextBlocks;

mod args;
mod auto_cache;
mod bird;
mod blog;
mod cache;
mod documents;
mod event;
mod lang;
mod looking_glass;
mod mirrors;
mod news;
mod peers;
mod posts;
mod routes;
mod state;
mod stats;
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

  println!("test");

  info!(concat!(
    "Booting ",
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    "..."
  ));

  println!("Normal Print!");

  let state = FoundationState {
    blog: Blogs::load(&args.content_directory.join("blog")).await?,
    news: News::load(&args.content_directory.join("news")).await?,
    text_blocks: TextBlocks::load(&args.content_directory.join("text_blocks"), &args.base_url)
      .await?,
    documents: Documents::load(&args.content_directory.join("documents")).await?,
    team: Team::load(&args.content_directory.join("team")).await?,
    stats: Stats::new(args.prometheus_url),
    peers: NetworkService::new(
      &args.content_directory.join("supporter"),
      args.ixp_manager_url,
    )
    .await?,
    bird: Bird::new(args.bird_html).await?,
    events: Events::load(&args.content_directory.join("event")).await?,
    mirrors: Mirrors::load(&args.content_directory.join("mirrors.yaml")).await?,
    looking_glass: LookingGlass::load(args.looking_glass_url).await?,
  };

  let stats = state.stats.clone();
  let looking_glass = state.looking_glass.clone();
  tokio::spawn(async move {
    loop {
      if let Err(err) = stats.update().await {
        error!("Failed to update stats: {:?}", err);
        tokio::time::sleep(Duration::from_secs(10)).await;
      } else {
        tokio::time::sleep(Duration::from_secs(60 * 10)).await;
      }
    }
  });

  tokio::spawn(async move {
    loop {
      if let Err(e) = looking_glass.routes.get().await {
        error!("error while updating routes cache: {e}");
      }

      tokio::time::sleep(Duration::from_secs(60 * 60)).await;
    }
  });

  let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
    .allow_headers([CONTENT_TYPE])
    .allow_origin(Any);

  let router = route(&ContentPaths {
    blog: args.content_directory.join("blog/assets"),
    event: args.content_directory.join("event/assets"),
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
