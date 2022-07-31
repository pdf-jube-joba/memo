use serde::{Serialize , Deserialize};
use std::hash::Hash;
use crate::shared;
use chrono::prelude::*;

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]
pub struct InfoSystem {
    pub created: chrono::DateTime<Local>,
    pub owner: shared::User
}

impl InfoSystem {
    pub fn from(created: chrono::DateTime<Local>, owner: shared::User) -> Self {
        Self {created, owner}
    }
    pub fn created_at(&self) -> &chrono::DateTime<Local> {
        &self.created
    }
    pub fn owner(&self) -> &shared::User{
        &self.owner
    }
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]
pub struct InfoUser {
    pub kind: Kind,
    pub origin: Origin,
    pub content_type: String
}

impl InfoUser {
    pub fn from(origin: Origin, kind: Kind, content_type: String) -> Self {
        Self {origin , kind , content_type}
    }
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]
pub enum Origin {
    Nothing,
    Creation(shared::Id),
    Download(shared::Id)
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]
pub enum Kind {
    Global,
    Local,
    UnderControl
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]
pub struct Body {
    link: String
}

impl Body {
    pub fn from(link: String) -> Self {
        Self {link}
    }
    pub fn link_mut(self: &mut Self) -> &mut String {
        &mut self.link
    }
    pub fn link(self: &Self) -> &String {
        &self.link
    }
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]

pub struct Constructor {
    pub registrant: shared::User,
    pub origin: Origin,
    pub kind: Kind,
    pub content_type: String,
    pub link: String
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]

pub enum Modifier {
    Origin(Origin),
//    Kind(Kind),
//    ContentType(String),
//    Link(String),
}