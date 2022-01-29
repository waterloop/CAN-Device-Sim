use super::common::*;
use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Producer {
    name: String,
    messages: Vec<OutMessage>,
    period: u32
}
