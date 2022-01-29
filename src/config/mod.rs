mod test {
    #[test]
    fn can_create_config() {
        let json = r#"{"info":{"subteam":"Firmware","dateCreated":"Sun Jan 23 2022","author":"Quinn"},"devices":[{"device_type":"producer","messages":[{"id":1,"data":[2]}],"period":10,"name":"Torchic"}]}"#;
        let config: super::Config = serde_json::from_str(json).unwrap();
    }

    #[test]
    fn get_producers() {
        let json = r#"{"info":{"subteam":"Firmware","dateCreated":"Sun Jan 23 2022","author":"Quinn"},"devices":[{"device_type":"producer","name":"Device2","period":100,"messages":[{"id":225,"data":[0]}]},{"device_type":"responder","hooks":[{"incommingMessage":{"id":0},"outgoingMessage":{"id":1,"data":[1]},"latency":20}],"name":"HelloImAResponder"},{"device_type":"producer","messages":[{"id":1,"data":[2]}],"period":10,"name":"Torchic"}]}"#;
        let json_devices = r#"[{"device_type":"producer","name":"Device2","period":100,"messages":[{"id":225,"data":[0]}]}, {"device_type":"producer","messages":[{"id":1,"data":[2]}],"period":10,"name":"Torchic"}]"#;
        let config: super::Config = serde_json::from_str(json).unwrap();
        let expected_producers: Vec<super::Device> = serde_json::from_str(json_devices).unwrap();
        let producers = config.get_producers();
        for (expected_producer, producer) in expected_producers.iter().zip(producers.iter()) {
            match expected_producer {
                super::Device::Producer(expected_producer) => {
                    assert_eq!(expected_producer, *producer);
                },
                _ => panic!("Found a device that wasn't a producer. This is a problem with the test code, not with the dut"),
            }
        }
    }
    #[test]
    fn get_responders() {
        let json = r#"{"info":{"subteam":"Firmware","dateCreated":"Sun Jan 23 2022","author":"Quinn"},"devices":[{"device_type":"producer","name":"Device2","period":100,"messages":[{"id":225,"data":[0]}]},{"device_type":"responder","hooks":[{"incommingMessage":{"id":0},"outgoingMessage":{"id":1,"data":[1]},"latency":20}],"name":"HelloImAResponder"},{"device_type":"producer","messages":[{"id":1,"data":[2]}],"period":10,"name":"Torchic"}]}"#;
        let json_devices = r#"[{"device_type":"responder","hooks":[{"incommingMessage":{"id":0},"outgoingMessage":{"id":1,"data":[1]},"latency":20}],"name":"HelloImAResponder"}]"#;
        let config: super::Config = serde_json::from_str(json).unwrap();
        let expected_responders: Vec<super::Device> = serde_json::from_str(json_devices).unwrap();
        let responders = config.get_responders();
        for (expected_responder, responder) in expected_responders.iter().zip(responders.iter()) {
            match expected_responder {
                super::Device::Responder(expected_responder) => {
                    assert_eq!(expected_responder, *responder);
                },
                _ => panic!("Found a device that wasn't a producer. This is a problem with the test code, not with the dut"),
            }
        }
    }
}

use serde::{ Serialize, Deserialize };
mod device;
pub use device::{
    Device,
    Producer,
    Responder
};


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Info {
    author: String,
    subteam: String,
    date_created: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    info: Info,
    devices: Vec<Device>
}

impl Config {
    /**
     * Returns the list of Producers from the device list
     */
    pub fn get_producers<'a>(&'a self) -> Vec<&'a Producer> {
        self.devices
            .iter()
            .filter(|x| matches!(x, Device::Producer(_)))
            .map(|x| if let Device::Producer(p) = x {
                p
            } else {
                panic!("Filter should prevent this from happening")
            })
            .collect()
    }
    /**
     * Returns the list of responders from the device list
     */
    pub fn get_responders<'a>(&'a self) -> Vec<&'a Responder> {
        self.devices
            .iter()
            .filter(|x| matches!(x, Device::Responder(_)))
            .map(|x| if let Device::Responder(p) = x {
                p
            } else {
                panic!("Filter should prevent this from happening")
            })
            .collect()
    }
}
