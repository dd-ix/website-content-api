use std::net::SocketAddr;

use clap::Parser;

#[derive(Parser)]
pub(crate) struct Args {
  #[clap(
    short,
    long,
    env = "META_LISTEN_ADDR",
    default_value = "127.0.0.1:8080"
  )]
  pub(crate) listen_addr: SocketAddr,
}
