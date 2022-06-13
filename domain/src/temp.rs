use serde::{Serialize , Deserialize};
use std::hash::Hash;
use crate::setting;
use chrono::prelude::*;

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]
pub struct InfoSystem {
  date: chrono::DateTime<Local>,
  owner: setting::User
}

impl InfoSystem {
  pub fn from(date: chrono::DateTime<Local>, owner: setting::User) -> Self {
      Self {date , owner}
  }
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]
pub enum Related {
  Temp(setting::Id),
  Link(setting::Id),
  Word(setting::Id),
  Article(setting::Id)
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]
pub struct InfoUser {
  related: Vec<Related>
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]
pub struct Body {
  memo: String
}