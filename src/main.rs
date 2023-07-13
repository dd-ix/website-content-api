use axum::Server;
use clap::Parser;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::args::Args;
use crate::news::News;
use crate::routes::route;
use crate::state::MetaState;

mod args;
mod lang;
mod news;
mod routes;
mod state;

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

  let state = MetaState {
    news: News::load(&args.content_directory.join("news")).await?,
  };

  let router = route().with_state(state);

  let server = Server::bind(&args.listen_addr).serve(router.into_make_service());

  info!("Listening on http://{}...", server.local_addr());

  if let Err(err) = server.await {
    error!("Error while serving api: {}", err);
  }

  Ok(())
}
