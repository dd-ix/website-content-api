use crate::bird::Bird;
use crate::blog::Blogs;
use crate::documents::Documents;
use crate::event::Events;
use crate::mirrors::Mirrors;
use crate::news::News;
use crate::peers::NetworkService;
use crate::stats::Stats;
use crate::team::Team;
use crate::text_blocks::TextBlocks;

#[derive(Clone)]
pub(crate) struct FoundationState {
  pub(crate) blog: Blogs,
  pub(crate) news: News,
  pub(crate) text_blocks: TextBlocks,
  pub(crate) documents: Documents,
  pub(crate) team: Team,
  pub(crate) stats: Stats,
  pub(crate) peers: NetworkService,
  pub(crate) bird: Bird,
  pub(crate) events: Events,
  pub(crate) mirrors: Mirrors,
}
