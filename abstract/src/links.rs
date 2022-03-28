use serde::{Serialize , Deserialize};
use std::hash::Hash;
use crate::setting;

#[derive(Debug , PartialEq , Eq)]
pub struct LinkObject {
    pub id: setting::Id,
    pub content: Content
}

impl LinkObject {
    pub fn new(id: setting::Id , content: Content) -> Self {
        LinkObject {id , content}
    }
    pub fn create(content: Content) -> Self {
        let id = setting::Id::create(&content);
        LinkObject {id , content}
    }
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq)]
pub struct Content {
    info: setting::Info,
    body: Body
}

impl Content {
    pub fn new(info: setting::Info , body: Body) -> Self {
        Content {body , info}
    }
    pub fn create(user: setting::User, body: Body) -> Self {
        let info = setting::Info::create(user);
        Content {info , body}

    }
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq)]
pub struct Body {
    kind: Kind,
    link: String
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq)]
pub enum Kind {
    Global,
    Local,
    UnderControl
}

impl Body {
    pub fn new(kind: Kind, link: String) -> Self {
        Self {kind , link}
    }
}
