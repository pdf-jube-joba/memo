use serde::{Serialize , Deserialize};
use std::hash::Hash;
use crate::{setting, link, word , article , temp};

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]
pub enum AccessError {
  NotFound,
  AlReadyExists
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]
pub struct Content<InfoSystem, InfoUser, Body> {
  info_system: InfoSystem,
  info_user: InfoUser,
  body: Body
}

impl<InfoSystem, InfoUser, Body> Content<InfoSystem, InfoUser, Body> {
  pub fn from(info_system: InfoSystem, info_user: InfoUser, body: Body) -> Self {
    Self {info_system, info_user, body}
  }
  pub fn info_system(&self) -> &InfoSystem {
    &self.info_system
  }
  pub fn info_user(&self) -> &InfoUser {
    &self.info_user
  }
  pub fn body(&self) -> &Body {
    &self.body
  }
}

pub type LinkContent = Content<link::InfoSystem, link::InfoUser, link::Body>;
pub type WordContent = Content<word::InfoSystem, word::InfoUser, word::Body>;
pub type ArticleContent = Content<article::InfoSystem, article::InfoUser, article::Body>;
pub type TempContent = Content<temp::InfoSystem, temp::InfoUser, temp::Body>;

pub trait LinkRepository {
  fn search(&self) -> Vec<setting::Id>;
  fn pick(&self, id: setting::Id) -> Result<LinkContent, AccessError>;
  fn modify(&mut self, id: setting::Id, info: link::InfoUser, body: link::Body) -> Result<(), AccessError>;
  fn post(&mut self, user: setting::User, info: link::InfoUser, body: link::Body) -> Result<(), AccessError>;
  fn delete(&mut self, id: setting::Id) -> Result<(), AccessError>;
}

pub trait WordRepository {
  fn search(&self) -> Vec<setting::Id>;
  fn pick(&self, id: setting::Id) -> Result<WordContent, AccessError>;
  fn modify(&mut self, id: setting::Id, info: word::InfoUser, body: word::Body) -> Result<(), AccessError>;
  fn post(&mut self, user: setting::User, info: word::InfoUser, body: word::Body) -> Result<(), AccessError>;
  fn delete(&mut self, id: setting::Id) -> Result<(), AccessError>;
}

pub trait ArticleRepository {
  fn search(&self) -> Vec<setting::Id>;
  fn pick(&self, id: setting::Id) -> Result<ArticleContent, AccessError>;
  fn modify(&mut self, id: setting::Id, info: article::InfoUser, body: article::Body) -> Result<(), AccessError>;
  fn post(&mut self, user: setting::User, info: article::InfoUser, word: article::Body) -> Result<(), AccessError>;
  fn delete(&mut self, id: setting::Id) -> Result<(), AccessError>;
}

pub trait TempRepository {
  fn search(&self) -> Vec<setting::Id>;
  fn pick(&self, id: setting::Id) -> Result<TempContent, AccessError>;
  fn modify(&mut self, id: setting::Id, info: temp::InfoUser, body: temp::Body) -> Result<(), AccessError>;
  fn post(&mut self, user: setting::User, info: temp::InfoUser, word: temp::Body) -> Result<(), AccessError>;
  fn delete(&mut self, id: setting::Id) -> Result<(), AccessError>;
}

pub struct Repository {
  pub link_repository: Box<dyn LinkRepository>,
  pub word_repository: Box<dyn WordRepository>,
  pub article_repository: Box<dyn ArticleRepository>,
  pub temp_repository: Box<dyn TempRepository>
}