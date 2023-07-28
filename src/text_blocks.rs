use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;

use anyhow::anyhow;
use regex::{Captures, Regex, Replacer};
use serde::Serialize;

use crate::lang::Language;

#[derive(Debug, Clone)]
pub(crate) struct TextBlocks {
  blocks: Arc<Vec<Arc<TextBlock>>>,
  foundation_listen_addr: SocketAddr,
}

#[derive(Serialize, Debug, Clone)]
pub(crate) struct TextBlock {
  slug: String,
  lang: Language,
  body: String,
}

struct AddressReplacer {
  foundation_listen_addr: SocketAddr,
}

impl Replacer for AddressReplacer {
  fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
    let file_name = caps.name("file").unwrap().as_str();

    dst.push_str(&*format!(
      "src=\"http://{}/text-blocks/assets/{}\"",
      self.foundation_listen_addr.to_string(),
      file_name,
    ))
  }
}

impl TextBlocks {
  pub(crate) async fn load(directory: &Path, foundation_listen_addr: SocketAddr) -> anyhow::Result<Self> {
    let mut blocks = Vec::new();

    let mut dir = tokio::fs::read_dir(directory).await?;
    while let Some(entry) = dir.next_entry().await? {
      if entry.file_type().await?.is_dir() {
        continue;
      }

      let path = entry.path();
      let body = tokio::fs::read_to_string(path.as_path()).await?;

      let file_name = path.file_stem().unwrap().to_str().unwrap();
      let (lang, slug) = parse_file_name(file_name)?;

      blocks.push(Arc::new(TextBlock {
        slug: slug.to_string(),
        lang,
        body: parse_markdown(&body, foundation_listen_addr)?,
      }));
    }


    Ok(TextBlocks {
      blocks: Arc::new(blocks),
      foundation_listen_addr,
    })
  }

  pub(crate) fn find_text_block(&self, lang: Language, slug: &str) -> Option<Arc<TextBlock>> {
    self
      .blocks
      .iter()
      .find(|block| block.lang == lang && block.slug == slug)
      .cloned()
  }
}

fn parse_file_name(file_name: &str) -> anyhow::Result<(Language, &str)> {
  let (slug, lang) = file_name
    .rsplit_once('.')
    .ok_or_else(|| anyhow!("Filename has a invalid format {}", file_name))?;

  Ok((lang.try_into()?, slug))
}

fn parse_markdown(markdown_body: &str, foundation_listen_addr: SocketAddr) -> anyhow::Result<String> {
  let html = markdown::to_html(markdown_body);
  let pattern = Regex::new("src=\"(?P<file>[^/]+)\"").unwrap();
  let modified_html = pattern.replace_all(&html, AddressReplacer {
    foundation_listen_addr,
  });


  Ok(modified_html.to_string())
}
