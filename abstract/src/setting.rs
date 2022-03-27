use serde::{Serialize , Deserialize};
use std::hash::{Hash};
use std::u64;

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq)]
pub enum User {
    Host ,
    Guest(u8)
}