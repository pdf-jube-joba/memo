use serde::{Serialize , Deserialize};
use std::hash::Hash;
use crate::setting;

#[derive(Serialize , Deserialize , Debug , PartialEq , Eq)]
pub struct WordObject {
    pub id: setting::Id,
    pub content: Content
}

impl WordObject {
    pub fn from(id: setting::Id , content: Content) -> Self {
        WordObject {id , content}
    }
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq)]
pub struct Content {
    info: Info,
    body: Body
}

impl Content {
    pub fn new(info: Info , body: Body) -> Self {
        Content {info  , body}
    }
    pub fn create(user: setting::User , body: Body) -> Self {
        let info = Info::new(user);
        Content {info , body}
    }
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq)]
pub struct Info {
    pub info_system: setting::InfoSystem,
    pub info_user: InfoUser
}

impl Info {
    pub fn new (user: setting::User) -> Self {
        Self {info_system: setting::InfoSystem::new(user) , info_user: InfoUser::new() }
    }
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq)]
pub struct InfoUser {
    pub manager: Vec::<setting::User>
}

impl InfoUser {
    pub fn new() -> Self {
        Self {manager: Vec::new()}
    }
    pub fn from(manager: Vec::<setting::User>) -> Self {
        Self {manager}
    }
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq)]
pub struct Body {
    title: String,
    description: String,
    main_content: Vec<TagObject>,
    sub_content: Vec<TagObject>
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq)]
pub struct TagObject {
    tag_type: String,
    tag_point: TagPoint
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq)]
pub enum TagPoint {
    Link(setting::Id),
    Word(setting::Id),
    Undefined(String)
}

impl Body {
    pub fn new(title: &String , description: &String) -> Self {
        Self {
            title: title.clone(),
            description: description.clone(),
            main_content: Vec::new(),
            sub_content: Vec::new()
        }
    }
    pub fn from(title: String, description: String, main_content: Vec<TagObject>, sub_content: Vec<TagObject>) -> Self {
        Body {title , description , main_content , sub_content}
    }
    pub fn push_main(self: &mut Self , t: TagObject) {
      self.main_content.push(t)
    }
    pub fn push_sub(self: &mut Self , t: TagObject) {
      self.sub_content.push(t)
    }
}