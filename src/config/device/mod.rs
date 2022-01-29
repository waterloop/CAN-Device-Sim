use serde::{ Serialize, Deserialize };

mod common;
mod producer;
mod responder;
pub use responder::Responder;
pub use producer::Producer;


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "device_type")]
pub enum Device {
    Producer (Producer),
    Responder (Responder)
}
