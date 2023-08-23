use crate::lang::Language;
use crate::state::FoundationState;
use crate::team::TeamMember;
use axum::extract::{Path, State};
use axum::Json;

pub(crate) async fn get_team(
  State(state): State<FoundationState>,
  Path(lang): Path<Language>,
) -> Json<Vec<TeamMember<String>>> {
  Json(state.team.members(&lang))
}
