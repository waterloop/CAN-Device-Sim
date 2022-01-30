use super::common::*;
use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Producer {
    pub name: String,
    pub messages: Vec<OutMessage>,
    pub period: u32
}
