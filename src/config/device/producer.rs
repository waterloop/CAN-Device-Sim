use super::common::*;
use serde::{ Serialize, Deserialize };

/**
 * @brief Producer: A producer sends messages in order with period denoted by the period field.
 */
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Producer {
    pub name: String,
    pub messages: Vec<OutMessage>,
    pub period: u32
}
