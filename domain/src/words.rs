use serde::{Serialize , Deserialize};
use std::hash::Hash;
use crate::setting;

#[derive(Debug , PartialEq , Eq)]
pub struct WordObject {
    id: setting::Id,
    content: Content
}

impl WordObject {
    pub fn new(id: setting::Id , content: Content) -> Self {
        WordObject {id , content}
    }
    pub fn create(content: Content) -> Self {
        let id = setting::Id::create(&content);
        WordObject {id , content}
    }
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq)]
pub struct Content {
    info: setting::Info,
    body: Body
}

impl Content {
    pub fn new(info: setting::Info , body: Body) -> Self {
        Content {info  , body}
    }
    pub fn create(user: setting::User , body: Body) -> Self {
        let info = setting::Info::create(user);
        Content {info , body}
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
    pub fn new(title: String, description: String, main_content: Vec<TagObject>, sub_content: Vec<TagObject>) -> Self {
        Body {title , description , main_content , sub_content}
    }
    pub fn create(title: &String , description: &String) -> Self {
      Self {
        title: title.clone(),
        description: description.clone(),
        main_content: Vec::new(),
        sub_content: Vec::new()
      }
    }
    pub fn push_main(self: &mut Self , t: TagObject) {
      self.main_content.push(t)
    }
    pub fn push_sub(self: &mut Self , t: TagObject) {
      self.sub_content.push(t)
    }
}