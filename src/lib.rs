
mod config {
    mod test {
        #[test]
        fn can_create_config() {
            let json = r#"{"info":{"subteam":"Firmware","dateCreated":"Sun Jan 23 2022","author":"Quinn"},"devices":[{"device_type":"producer","messages":[{"id":1,"data":[2]}],"period":10,"name":"Torchic"}]}"#;
            let config: super::Config = serde_json::from_str(json).unwrap();
        }
    }
    use serde::{ Serialize, Deserialize };
    #[derive(Debug, Serialize, Deserialize)]
    struct Info {
        author: String,
        subteam: String,
        dateCreated: String
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct InMessage {
        id: u8
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct OutMessage {
        id: u8,
        data: Vec<u8>
    }


    #[derive(Debug, Serialize, Deserialize)]
    struct Hook {
        incommingMessage: InMessage,
        latency: u32,
        outgoingMessage: OutMessage
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase", tag = "device_type")]
    enum Device {
        Producer {
            name: String,
            messages: Vec<OutMessage>,
            period: u32,
        },
        Responder {
            hooks: Vec<Hook>
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Config {
        info: Info,
        devices: Vec<Device>
    }

}
