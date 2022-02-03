use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct InMessage {
    pub id: u32
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct OutMessage {
    pub id: u32,
    pub data: Vec<u8>
}


#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Hook {
    pub incomming_message: InMessage,
    pub latency: u32,
    pub outgoing_message: OutMessage
}
