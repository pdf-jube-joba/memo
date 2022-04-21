use serde::{Serialize , Deserialize};
use std::hash::Hash;
use crate::setting;

#[derive(Serialize , Deserialize , Debug , PartialEq , Eq)]
pub struct LinkObject {
    id: setting::Id,
    content: Content
}

impl LinkObject {
    pub fn from(id: setting::Id , content: Content) -> Self {
        Self {id , content}
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
    info: Info,
    body: Body
}

impl Content {
    pub fn from_ib(info: Info , body: Body) -> Self {
        Content {body , info}
    }
/*
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
*/
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]
pub struct Info {
    info_system: setting::InfoSystem,
    info_user: InfoUser
}

impl Info {
    pub fn from(info_system: setting::InfoSystem , info_user: InfoUser) -> Self {
        Self {info_system , info_user}
    }
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]
pub struct InfoUser {
    origin: Origin,
    kind: Kind
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
}

pub enum LinkError {
    NotFound,
    AlReadyExists
}

pub trait LinkRepository {
    fn listup(&self) -> Vec<setting::Id>;
    fn search(&self , id: setting::Id) -> Result<LinkObject , LinkError>;
    fn modify(&mut self , id: setting::Id , content: Content) -> Result<() , LinkError>;
    fn post(&mut self , content: Content) -> Result<() , LinkError>;
}