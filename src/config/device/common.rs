use serde::{ Serialize, Deserialize };

/**
 * @brief InMessage: An in message represents a filter for incomming messages.
 * A device may filter incomming messages based on the id of the message.
 */
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct InMessage {
    pub id: u32
}

/**
 * @brief OutMessage: An out message is the format of a message sent from a device. The id
 * will be the id of the message sent, the data field should not be more then 4 bytes to fit
 * into a can message.
 */
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct OutMessage {
    pub id: u32,
    pub data: Vec<u8>
}

/**
 * @brief Hook: A hook is a combination of an incomming filter, a latency and a response.
 * When a device is registered with a hook, it will listen for the incoming message and then
 * it will respond with the the outgoing message after an amount of time denoted by latency.
 */
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Hook {
    pub incomming_message: InMessage,
    pub latency: u32,
    pub outgoing_message: OutMessage
}
