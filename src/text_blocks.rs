use std::path::Path;
use std::sync::Arc;

use anyhow::anyhow;
use regex::{Captures, Regex, Replacer};
use serde::Serialize;
use url::Url;

use crate::lang::Language;

#[derive(Debug, Clone)]
pub(crate) struct TextBlocks {
  blocks: Arc<Vec<Arc<TextBlock>>>,
}

#[derive(Serialize, Debug, Clone)]
pub(crate) struct TextBlock {
  slug: String,
  lang: Language,
  body: String,
}

struct AddressReplacer {
  base_url: Url,
}

impl Replacer for AddressReplacer {
  fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
    let file_name = caps.name("file").unwrap().as_str();

    dst.push_str(&format!(
      "src=\"{}\"",
      self.base_url.join(file_name).unwrap(),
    ))
  }
}

impl TextBlocks {
  pub(crate) async fn load(directory: &Path, base_url: &Url) -> anyhow::Result<Self> {
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
        body: parse_markdown(&body, base_url.join("/text-blocks/assets/")?)?,
      }));
    }

    Ok(TextBlocks {
      blocks: Arc::new(blocks),
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

fn parse_markdown(markdown_body: &str, base_url: Url) -> anyhow::Result<String> {
  let html = markdown::to_html(markdown_body);
  let pattern = Regex::new("src=\"(?P<file>[^\"]+)\"").unwrap();
  let modified_html = pattern.replace_all(&html, AddressReplacer { base_url });

  Ok(modified_html.to_string())
}
