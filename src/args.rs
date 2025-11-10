use std::net::SocketAddr;
use std::path::PathBuf;

use clap::Parser;
use url::Url;

#[derive(Parser)]
pub(crate) struct Args {
  #[clap(
    short,
    long,
    env = "WEBSITE_CONTENT_API_LISTEN_ADDR",
    default_value = "127.0.0.1:8080"
  )]
  pub(crate) listen_addr: SocketAddr,

  #[clap(
    short,
    long,
    env = "WEBSITE_CONTENT_API_CONTENT_DIRECTORY",
    default_value = "content"
  )]
  pub(crate) content_directory: PathBuf,

  #[clap(
    short,
    long,
    env = "WEBSITE_CONTENT_API_BASE_URL",
    default_value = "http://localhost:8080/"
  )]
  pub(crate) base_url: Url,

  #[clap(
    long,
    env = "WEBSITE_CONTENT_API_PROMETHEUS_URL",
    default_value = "https://svc-prom01.dd-ix.net/"
  )]
  pub(crate) prometheus_url: Url,

  #[clap(
    long,
    env = "WEBSITE_CONTENT_API_IXP_MANAGER_URL",
    default_value = "https://portal.dd-ix.net/"
  )]
  pub(crate) ixp_manager_url: Url,

  #[clap(
    long,
    env = "WEBSITE_CONTENT_API_ALICE_LOOKING_GLASS_URL",
    default_value = "https://lg.dd-ix.net/"
  )]
  pub(crate) alice_looking_glass_address: Url,

  #[clap(
    long,
    env = "WEBSITE_CONTENT_API_BIRD_HTML",
    default_value = "/var/lib/website-content-api/bird.html"
  )]
  pub(crate) bird_html: PathBuf,
}
