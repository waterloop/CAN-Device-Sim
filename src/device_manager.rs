mod test {
    use super::*;
}
use socketcan::CANSocket;
use crate::config;

struct Worker<'config> {
    handle: std::thread::JoinHandle<()>,
    sender: std::sync::mpsc::Sender,
}

struct INITIALIZATION;
struct RUNNING;
struct STOPPED;
/**
 * The config Lifetime should be the same as the config object which the producers and the responders are added from
 */
pub struct DeviceManager<'config, State=INTIALIZATION> {
    can_socket_handle: CANSocket,
    producers: Vec<&'config config::Producer>,
    responders: Vec<&'config config::Responder>,
    workers: Vec<Worker>
    state: std::marker::PhantomData<State>
}

impl<'config, State> DeviceManager<'config, State> {
    ///
    /// Create a new DeviceManager for a Can Interface. You can use the scripts/start-vcan0.sh script to start the vcan0 interface on your device.
    ///  ```
    /// use virtual_can_device::device_manager::DeviceManager;
    /// let device_manager = DeviceManager::new("vcan0").unwrap();
    /// ```
    ///
    pub fn new(can_interface: &str) -> Result<DeviceManager<INTIALIZATION>, Box<dyn std::error::Error>> {
        let can_socket_handle = CANSocket::open(can_interface)?;
        Ok(DeviceManager {
            can_socket_handle,
            producers: Vec::new(),
            responders: Vec::new(),
            state: std::marker::PhantomData
        })
    }
}

impl <'config> for DeviceManager<'config, INITIALIZATION> {
       ///
    /// ADD A producer to the list of producers
    /// ```
    /// use virtual_can_device::device_manager::DeviceManager;
    /// use virtual_can_device::config::Config;
    /// let mut device_manager = DeviceManager::new("vcan0").unwrap();
    /// let json = r#"{"info":{"subteam":"Firmware","dateCreated":"Sun Jan 23 2022","author":"Quinn"},"devices":[{"device_type":"producer","name":"Device2","period":100,"messages":[{"id":225,"data":[0]}]},{"device_type":"responder","hooks":[{"incommingMessage":{"id":0},"outgoingMessage":{"id":1,"data":[1]},"latency":20}],"name":"HelloImAResponder"},{"device_type":"producer","messages":[{"id":1,"data":[2]}],"period":10,"name":"Torchic"}]}"#;
    /// let config: Config = serde_json::from_str(json).unwrap();
    /// for producer in config.get_producers() {
    ///     device_manager.add_producer(producer);
    /// }
    ///
    /// ```
    pub fn add_producer(&mut self, producer: &'config config::Producer) {
        self.producers.push(producer);
     }
    ///
    /// ADD A responder to the list of responders
    /// ```
    /// use virtual_can_device::device_manager::DeviceManager;
    /// use virtual_can_device::config::Config;
    /// let mut device_manager = DeviceManager::new("vcan0").unwrap();
    /// let json = r#"{"info":{"subteam":"Firmware","dateCreated":"Sun Jan 23 2022","author":"Quinn"},"devices":[{"device_type":"producer","name":"Device2","period":100,"messages":[{"id":225,"data":[0]}]},{"device_type":"responder","hooks":[{"incommingMessage":{"id":0},"outgoingMessage":{"id":1,"data":[1]},"latency":20}],"name":"HelloImAResponder"},{"device_type":"producer","messages":[{"id":1,"data":[2]}],"period":10,"name":"Torchic"}]}"#;
    /// let config: Config = serde_json::from_str(json).unwrap();
    /// for responder in config.get_responders() {
    ///     device_manager.add_responder(responder);
    /// }
    ///
    /// ```
    pub fn add_responder(&mut self, responder: &'config config::Responder) {
        self.responders.push(responder);
    }

    pub fn start_devices(mut self) -> DeviceManager<'config, RUNNING> {
        for producer in self.producers {
            self.workers.push
        }
    }
}

impl <'config> DeviceManager<'config, RUNNING> {
    pub fn stop_devices(&self) -> DeviceManager<'config, STOPPED>{ todo!() }
}
