use super::common::*;
use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Responder {
    name: String,
    hooks: Vec<Hook>
}
