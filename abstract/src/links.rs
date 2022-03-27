use serde::{Serialize , Deserialize};
use chrono::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash , Hasher};
use std::u64;
use crate::setting;

#[derive(Debug , PartialEq , Eq)]
pub struct LinkObject {
    pub id: Id,
    pub content: Content
}

impl LinkObject {
    pub fn new(content: Content) -> Self {
        let id = Id::new(&content);
        LinkObject {id , content}
    }
}

#[derive(Serialize , Deserialize , Debug , PartialEq , Eq)]
pub struct Id {
    id: u64,
}

impl Id {
    pub fn new(content: &Content) -> Self {
        let mut hash = DefaultHasher::new();
        content.hash(&mut hash);
        Id {id: hash.finish()}
    }
    pub fn temp(id: &String) -> Result<Self , Box<dyn std::error::Error>> {
        let id = u64::from_str_radix(id , 16)?;
        Ok(Id{id})
    }
    pub fn id(self: &Self) -> String {
        format!("{:018x?}", self.id)
    }
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq)]
pub struct Content {
    info: Info,
    body: Body
}

impl Content {
    pub fn new(info: Info , body: Body) -> Self {
        Content {body , info}
    }
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq)]
pub struct Info {
    date: chrono::DateTime<Local>,
    registrant: setting::User
}

impl Info {
    pub fn create(user: setting::User) -> Self {
        Info {date: Local::now() , registrant: user}
    }
    pub fn new(date: chrono::DateTime<Local> , user: setting::User) -> Self {
        Info {date , registrant: user}
    }
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq)]
pub struct Body {
    kind: Kind,
    link: String
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq)]
pub enum Kind {
    InWeb,
    InLocal,
    InThisDir
}

impl Body {
    pub fn new(local: bool, link: String) -> Self {
        if local {
            Body {kind: Kind::InLocal , link}
        } else {
            Body {kind: Kind::InWeb , link}
        }
    }
}
