use serde::{Serialize , Deserialize};
use std::hash::Hash;
use std::u64;

#[derive(Serialize , Deserialize , Debug , Hash , PartialEq , Eq , Clone)]
pub struct User {
  pub number: u8
}