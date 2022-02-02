use super::common::*;
use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Responder {
    pub name: String,
    pub hook: Hook
}
