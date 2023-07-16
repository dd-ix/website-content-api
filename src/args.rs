use std::net::SocketAddr;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub(crate) struct Args {
  #[clap(
    short,
    long,
    env = "FOUNDATION_LISTEN_ADDR",
    default_value = "127.0.0.1:8080"
  )]
  pub(crate) listen_addr: SocketAddr,

  #[clap(
    short,
    long,
    env = "FOUNDATION_CONTENT_DIRECTORY",
    default_value = "content"
  )]
  pub(crate) content_directory: PathBuf,
}
