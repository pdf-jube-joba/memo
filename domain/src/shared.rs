use serde::{Serialize , Deserialize};
use std::hash::Hash;
use std::u64;

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]
pub struct User {
  pub number: u8
}

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