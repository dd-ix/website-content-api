use crate::bird::Bird;
use crate::blog::Blog;
use crate::documents::Documents;
use crate::event::EventHandler;
use crate::peers::NetworkService;
use crate::stats::Stats;
use crate::team::Team;
use crate::text_blocks::TextBlocks;

#[derive(Clone)]
pub(crate) struct FoundationState {
  pub(crate) blog: Blog,
  pub(crate) text_blocks: TextBlocks,
  pub(crate) documents: Documents,
  pub(crate) team: Team,
  pub(crate) stats: Stats,
  pub(crate) peers: NetworkService,
  pub(crate) bird: Bird,
  pub(crate) events: EventHandler,
}
