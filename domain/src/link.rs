use serde::{Serialize , Deserialize};
use std::hash::Hash;
use crate::setting;
use chrono::prelude::*;

/*
#[derive(Serialize , Deserialize , Debug , PartialEq , Eq)]
pub struct LinkObject {
    id: setting::Id,
    content: Content
}

impl LinkObject {
    pub fn from(id: setting::Id , content: Content) -> Self {
        Self {id , content}
    }
    pub fn content(self: &mut Self) -> &Content {
        &self.content
    }
    pub fn id(self: &mut Self) -> &setting::Id {
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
        Self {info_system, info_user, body}
    }
    pub fn info_system(self: &mut Self) -> &setting::InfoSystem {
        &self.info_system
    }
    pub fn info_user(self: &mut Self) -> &InfoUser {
        &self.info_user
    }
    pub fn body(self: &mut Self) -> &Body {
        &self.body
    }

    pub fn from(
        user: setting::User,
        origin: Origin,
        kind: Kind,
        name: String,
        content_type: String,
        link: String
    ) -> Self {
        let info = Info::new(user , origin , kind);
        let body = Body::new(name , content_type , link);
        Content {info , body}
    }

}
*/

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
pub struct InfoUser {
    origin: Origin,
    kind: Kind
}

impl InfoUser {
    pub fn from(origin: Origin, kind: Kind) -> Self {
        Self {origin , kind}
    }
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]
pub enum Origin {
    Nothing,
    Creation(setting::Id),
    Download(setting::Id)
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]
pub enum Kind {
    Global,
    Local,
    UnderControl
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]
pub struct Body {
    name: String,
    content_type: String,
    link: String
}

impl Body {
    pub fn from(name: String, content_type: String, link: String) -> Self {
        Self {name, content_type, link}
    }
    pub fn name(self: &mut Self) -> &String {
        &mut self.name
    }
    pub fn content_type(self: &mut Self) -> &String {
        &mut self.content_type
    }
    pub fn link(self: &mut Self) -> &String {
        &mut self.link
    }
}