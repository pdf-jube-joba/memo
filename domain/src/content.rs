use serde::{Serialize , Deserialize};
use std::hash::Hash;
use crate::shared;
use chrono::prelude::*;

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]
pub struct InfoSystem {
    created: chrono::DateTime<Local>,
    owner: shared::User
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]
pub struct InfoUser {
    manager: Vec::<shared::User>
}

impl InfoSystem {
    pub fn from(created: chrono::DateTime<Local>, owner: shared::User) -> Self {
        Self {created , owner}
    }
}

impl InfoUser {
    pub fn from(manager: Vec::<shared::User>) -> Self {
        Self {manager}
    }
    pub fn manager(self: &mut Self) -> &mut Vec::<shared::User> {
        &mut self.manager
    }
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]
pub struct Body {
    title: String,
    link: Vec<(String, TagObject)>,
    tag: Vec<TagObject>
}

impl Body {
    pub fn from(title: String, link: Vec<(String,TagObject)>, tag: Vec<TagObject>) -> Self {
        Self {title , link , tag}
    }
    pub fn title(self: &mut Self) -> &mut String {
        &mut self.title
    }
    pub fn link(self: &mut Self) -> &mut Vec<(String, TagObject)> {
        &mut self.link
    }
    pub fn tag(self: &mut Self) -> &mut Vec<TagObject> {
        &mut self.tag
    }
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]
pub struct TagObject {
    tag_type: String,
    tag_point: TagPoint
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]
pub enum TagPoint {
    Link(shared::Id),
    Undefined(String)
}

pub struct Constructor {
    registrant: shared::User,
    title: String,
    link: Vec<(String, TagObject)>,
    tag: Vec<TagObject>
}

pub enum Modifier {
    Title(String),
    Link(Vec<(String, TagObject)>)
}