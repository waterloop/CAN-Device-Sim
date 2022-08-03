use serde::{ Serialize, Deserialize };

mod producer;
mod responder;
pub mod common;
pub use responder::Responder;
pub use producer::Producer;


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "device_type")]
pub enum Device {
    Producer (Producer),
    Responder (Responder)
}
