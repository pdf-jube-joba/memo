use chrono::prelude::*;
use ownlinkmemo_domain as domain;
use domain::{repository as repository};

pub const LEFTPANEL_LENGTH : f32 = 150.0;
pub const BUTTONPANEL_LENGTH : f32 = 50.0;
pub const TILE_LENGTH : f32 = 100.0;

pub enum MessageResponse {
  Search,
  Pick(repository::Id),
  Post(repository::Constructor),
  Modify(repository::Id, repository::Modifier)
}

pub enum AppStateChangeRequest {
  BackHome,
  Reload(Vec<(repository::Id, repository::Memo, Thumbnail)>),
  ViewMemo(repository::Id, repository::Memo),
}

pub enum InnerMessage {
  BackHome,
  MoveTo(domain::repository::Id),
  CreateLink(CreateLink),
  MakingView,
}

pub struct CreateLink {
  pub link: String,
}

pub enum View {
  Home,
  InMemoLoad,
  InMemo(repository::Id, MemoTemp),
  CreateLink(CreateLink),
}

impl Default for View {
  fn default() -> Self {
      View::Home
  }
}

pub enum Thumbnail {
  Loading(Option<u8>),
  Default,
}

pub enum MemoTemp {
  Link(MemoTempLink),
}

pub struct MemoTempLink {
  pub time: chrono::DateTime<Local>,
  pub owner: Option<domain::shared::User>,
  pub link: String,
  pub link_changeable: bool,
}