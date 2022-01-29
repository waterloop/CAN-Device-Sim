use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InMessage {
    id: u8
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct OutMessage {
    id: u8,
    data: Vec<u8>
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Hook {
    incomming_message: InMessage,
    latency: u32,
    outgoing_message: OutMessage
}
