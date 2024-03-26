use crate::documents::Documents;
use crate::lists::MailingLists;
use crate::news::News;
use crate::peers::NetworkService;
use crate::stats::Stats;
use crate::team::Team;
use crate::text_blocks::TextBlocks;

#[derive(Clone)]
pub(crate) struct FoundationState {
  pub(crate) news: News,
  pub(crate) text_blocks: TextBlocks,
  pub(crate) documents: Documents,
  pub(crate) team: Team,
  pub(crate) lists: MailingLists,
  pub(crate) stats: Stats,
  pub(crate) peers: NetworkService,
}
