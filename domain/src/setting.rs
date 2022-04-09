use serde::{Serialize , Deserialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash , Hasher};
use std::u64;
use chrono::prelude::*;

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq)]
pub enum User {
    Host ,
    Guest(u8)
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq)]
pub struct Info {
    date: chrono::DateTime<Local>,
    owner: User,
    manager: Vec<User>
}

impl Info {
    pub fn create(user: User) -> Self {
        Info {date: Local::now() , owner: user , manager: Vec::new()}
    }
    pub fn new(date: chrono::DateTime<Local> , user: User) -> Self {
        Info {date , owner: user , manager: Vec::new()}
    }
}

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq)]
pub struct Id {
    id: u64,
}

impl Id {
    pub fn create<T: Hash>(content: &T) -> Self {
        let mut hash = DefaultHasher::new();
        content.hash(&mut hash);
        Id {id: hash.finish()}
    }
    pub fn id_or(id: &String) -> Result<Self , Box<dyn std::error::Error>> {
        let id = u64::from_str_radix(id , 16)?;
        Ok(Id{id})
    }
    pub fn id(self: &Self) -> String {
        format!("{:018x?}", self.id)
    }
}