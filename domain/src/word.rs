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
    description: String,
    reference: Vec<TagObject>,
    tag: Vec<TagObject>
}

impl Body {
    pub fn from(title: String, description: String, reference: Vec<TagObject>, tag: Vec<TagObject>) -> Self {
        Self {title , description , reference , tag}
    }
    pub fn title(self: &mut Self) -> &mut String {
        &mut self.title
    }
    pub fn description(self: &mut Self) -> &mut String {
        &mut self.description
    }
    pub fn reference(self: &mut Self) -> &mut Vec<TagObject> {
        &mut self.reference
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
    Link(Id),
    Word(Id),
    Undefined(String)
}

pub struct Constructor {
    registrant: shared::User,
    title: String
}

pub enum Modifier {
    Manager(Vec<shared::User>),
    Title(String),
    Description(String),
    Reference(Vec<TagObject>),
    Tag(Vec<TagObject>),
}