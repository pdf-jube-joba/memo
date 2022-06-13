use serde::{Serialize , Deserialize};
use std::hash::Hash;
use crate::setting;
use chrono::prelude::*;

/*
#[derive(Serialize , Deserialize , Debug , PartialEq , Eq , Clone)]
pub struct WordObject {
    id: setting::Id,
    content: Content
}

impl WordObject {
    pub fn from(id: setting::Id , content: Content) -> Self {
        WordObject {id , content}
    }
    pub fn content(self: &Self) -> &Content {
        &self.content
    }
    pub fn id(self: &Self) -> &setting::Id {
        &self.id
    }
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]
pub struct Content {
    info_system: setting::InfoSystem,
    info_user: InfoUser,
    body: Body
}

impl Content {
    pub fn from(info_system: setting::InfoSystem, info_user: InfoUser, body: Body) -> Self {
        Self {info_system, info_user , body}
    }
    pub fn info_system(self: &Self) -> &setting::InfoSystem {
        &self.info_system
    }
    pub fn info_user(self: &Self) -> &InfoUser {
        &self.info_user
    }
    pub fn body(self: &Self) -> &Body {
        &self.body
    }
}
*/

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]
pub struct InfoSystem {
    date: chrono::DateTime<Local>,
    owner: setting::User
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]
pub struct InfoUser {
    manager: Vec::<setting::User>
}

impl InfoSystem {
    pub fn from(date: chrono::DateTime<Local>, owner: setting::User) -> Self {
        Self {date , owner}
    }
}

impl InfoUser {
    pub fn from(manager: Vec::<setting::User>) -> Self {
        Self {manager}
    }
    pub fn manager(self: &mut Self) -> &mut Vec::<setting::User> {
        &mut self.manager
    }
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]
pub struct Body {
    title: String,
    description: String,
    linkto: Vec<TagObject>,
    tag: Vec<TagObject>
}

impl Body {
    pub fn from(title: String, description: String, linkto: Vec<TagObject>, tag: Vec<TagObject>) -> Self {
        Self {title , description , linkto , tag}
    }
    pub fn title(self: &mut Self) -> &mut String {
        &mut self.title
    }
    pub fn description(self: &mut Self) -> &mut String {
        &mut self.description
    }
    pub fn linkto(self: &mut Self) -> &mut Vec<TagObject> {
        &mut self.linkto
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
    Link(setting::Id),
    Word(setting::Id),
    Undefined(String)
}