use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct InMessage {
    id: u8
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct OutMessage {
    pub id: u32,
    pub data: Vec<u8>
}


#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Hook {
    incomming_message: InMessage,
    latency: u32,
    outgoing_message: OutMessage
}
