use serde::{Serialize , Deserialize};
use std::hash::Hash;
use crate::shared;
use chrono::prelude::*;

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]
pub struct Id {
    pub id: u64,
}

impl Id {
    pub fn from(id: u64) -> Self {
        Self {id}
    }
    pub fn to_string(id: &Id) -> String {
        id.id.to_string()
    }
    pub fn id_or(id: &String) -> Result<Self , Box<dyn std::error::Error>> {
        let id = u64::from_str_radix(id , 16)?;
        Ok(Self{id})
    }
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]
pub struct InfoSystem {
  created: chrono::DateTime<Local>,
  owner: shared::User
}

impl InfoSystem {
  pub fn from(created: chrono::DateTime<Local>, owner: shared::User) -> Self {
      Self {created , owner}
  }
}

/*
#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]
pub enum Related {
  Temp(shared::Id),
  Link(shared::Id),
  Word(shared::Id),
  Article(shared::Id)
}
*/

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]
pub struct InfoUser {
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]
pub struct Body {
  memo: String
}

pub struct Constructor {

}

pub enum Modifier {

}