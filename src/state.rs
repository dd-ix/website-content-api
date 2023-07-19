use crate::documents::Documents;
use crate::static_content::StaticContent;
use crate::news::News;

#[derive(Clone)]
pub(crate) struct FoundationState {
  pub(crate) news: News,
  pub(crate) documents: Documents,
  pub(crate) static_content: StaticContent,
}
