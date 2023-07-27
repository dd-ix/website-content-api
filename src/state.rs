use crate::documents::Documents;
use crate::news::News;
use crate::text_blocks::TextBlocks;

#[derive(Clone)]
pub(crate) struct FoundationState {
  pub(crate) news: News,
  pub(crate) text_blocks: TextBlocks,
  pub(crate) documents: Documents,
}
