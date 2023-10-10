use crate::lang::Language;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Clone)]
pub(crate) enum WorkingGroup {
  Technical,
  Network,
  Service,
  DevOps,
  Events,
  FinancesAndLaw,
  ClientsAndSponsors,
  PublicRelations,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Socials {
  pub github: Option<String>,
  pub email: Option<String>,
  pub mastodon: Option<String>,
  pub website: Option<String>,
  pub linkedin: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct TeamMember<T> {
  pub name: String,
  pub nick: Option<String>,
  pub vorstand: bool,
  pub teams: Vec<WorkingGroup>,
  pub ripe_handle: Option<String>,
  pub description: T,
  pub image: String,
  pub socials: Socials,
}

#[derive(Clone)]
pub(crate) struct Team {
  members: Arc<Vec<TeamMember<HashMap<Language, String>>>>,
}

impl Team {
  pub(crate) async fn load(path: &Path) -> anyhow::Result<Self> {
    let base_path: PathBuf = path.into();
    let serialized_teams = tokio::fs::read_to_string(base_path.join("team.yaml")).await?;
    let team_members = serde_yaml::from_str(&serialized_teams)?;
    Ok(Self {
      members: team_members,
    })
  }

  pub(crate) fn members(&self, lang: &Language) -> Vec<TeamMember<String>> {
    self
      .members
      .iter()
      .map(|member| TeamMember {
        name: member.name.clone(),
        nick: member.nick.clone(),
        vorstand: member.vorstand.clone(),
        teams: member.teams.clone(),
        ripe_handle: member.ripe_handle.clone(),
        description: member.description.get(lang).unwrap().clone(),
        image: member.image.clone(),
        socials: member.socials.clone(),
      })
      .collect()
  }
}
