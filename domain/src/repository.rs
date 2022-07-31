use serde::{Serialize , Deserialize};
use std::hash::Hash;
use crate::{shared, link, word , article , temp, content};

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]
pub enum AccessError {
    NotFound,
    AlReadyExists
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]
pub struct Object<InfoSystem, InfoUser, Body> {
    info_system: InfoSystem,
    info_user: InfoUser,
    body: Body
}

impl<InfoSystem, InfoUser, Body> Object<InfoSystem, InfoUser, Body> {
    pub fn from(info_system: InfoSystem, info_user: InfoUser, body: Body) -> Self {
        Self {info_system, info_user, body}
    }
    pub fn info_system_mut(&mut self) -> &mut InfoSystem {
        &mut self.info_system
    }
    pub fn info_system(&self) -> &InfoSystem {
        & self.info_system
    }
    pub fn info_user_mut(&mut self) -> &mut InfoUser {
        &mut self.info_user
    }
    pub fn info_user(&self) -> &InfoUser {
        &self.info_user
    }
    pub fn body_mut(&mut self) -> &mut Body {
       &mut self.body
    }
    pub fn body(&self) -> &Body {
       &self.body
    }
}

pub type LinkObject = Object<link::InfoSystem, link::InfoUser, link::Body>;
pub type ContentObject = Object<content::InfoSystem, content::InfoUser, content::Body>;
pub type WordObject = Object<word::InfoSystem, word::InfoUser, word::Body>;
pub type ArticleObject = Object<article::InfoSystem, article::InfoUser, article::Body>;
pub type TempObject = Object<temp::InfoSystem, temp::InfoUser, temp::Body>;

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]

pub enum Memo {
    Link(LinkObject),
    Word(WordObject),
    Content(WordObject),
    Article(ArticleObject),
    Temp(TempObject)
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]

pub enum Id {
    Link(shared::Id),
    Content(shared::Id),
    Word(shared::Id),
    Article(shared::Id),
    Temp(shared::Id)
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]

pub enum Constructor {
    Link(link::Constructor)
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]

pub enum Modifier {
    Link(link::Modifier)
}

pub trait MemoRepository {
    fn search(&self) -> Vec<Id>;
    fn pick(&self, id: Id) -> Result<Memo, AccessError>;
    fn post(&mut self, constructor: Constructor) -> Result<(), AccessError>;
    fn modify(&mut self, id: Id, modifier: Modifier) -> Result<(), AccessError>;
//    fn delete(&mut self, id: Id) -> Result<(), AccessError>;
}