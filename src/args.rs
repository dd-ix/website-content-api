use std::net::SocketAddr;
use std::path::PathBuf;

use clap::Parser;
use url::Url;

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

  #[clap(
    short,
    long,
    env = "FOUNDATION_BASE_URL",
    default_value = "http://localhost:8080/"
  )]
  pub(crate) base_url: Url,

  #[clap(
    short,
    long,
    env = "FOUNDATION_LISTMONK_URL",
    default_value = "http://localhost:8090/"
  )]
  pub(crate) listmonk_url: Url,

  #[clap(
    short,
    long,
    env = "FOUNDATION_LISTMONK_USER",
    default_value = "listmonk"
  )]
  pub(crate) listmonk_user: String,

  #[clap(
    short,
    long,
    env = "FOUNDATION_LISTMONK_PASSWORD_FILE",
    default_value = "/run/secret/listmonk"
  )]
  pub(crate) listmonk_password_file: PathBuf,

  #[clap(short, long, env = "FOUNDATION_LISTMONK_LISTS", default_value = "[]")]
  pub(crate) listmonk_lists: String,
}
