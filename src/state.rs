use crate::documents::Documents;
use crate::news::News;

#[derive(Clone)]
pub(crate) struct FoundationState {
  pub(crate) news: News,
  pub(crate) documents: Documents,
}
