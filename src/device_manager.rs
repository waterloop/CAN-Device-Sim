mod test {
    use super::*;
}
use socketcan::CANSocket;
use crate::config;

enum WorkerMessage {
    Stop
}

trait StartDevice {
    fn start(&self, can_interface: &str) -> Worker;
}

impl StartDevice for &config::Producer {
    fn start(&self, can_interface: &str) -> Worker {
        let (sender, receiver) = std::sync::mpsc::channel::<WorkerMessage>();
        let period = self.period;
        let messages: Vec<socketcan::CANFrame> = self.messages.iter().map(|message| socketcan::CANFrame::new(message.id, &message.data, false, false).unwrap()).collect();

        let can_handle = CANSocket::open(can_interface).unwrap();
        let handle = std::thread::Builder::new().name(format!("[PRODUCER THREAD]: {}", self.name)).spawn(move | | {
            let mut message_counter = 0;
            loop {
                std::thread::sleep(std::time::Duration::from_millis(period.into()));
                match receiver.try_recv() {
                    Ok(WorkerMessage::Stop) => return,
                    Err(std::sync::mpsc::TryRecvError::Disconnected) => return,
                    Err(std::sync::mpsc::TryRecvError::Empty) => {},
                }
                can_handle.write_frame(&messages[message_counter]).unwrap();
                message_counter = (message_counter + 1) % messages.len();
            }
        }).unwrap();
        Worker {
            handle,
            sender
        }
    }
}
const CAN_SFF_MASK: u32 = 0x000007FF; // https://docs.huihoo.com/doxygen/linux/kernel/3.7/can_8h.html
impl StartDevice for &config::Responder {
    fn start(&self, can_interface: &str) -> Worker {
        let (sender, receiver) = std::sync::mpsc::channel::<WorkerMessage>();
        let ref hook = self.hook;
        let latency = hook.latency;
        let can_handle = CANSocket::open(can_interface).unwrap();
        let out_message = socketcan::CANFrame::new(hook.outgoing_message.id, &hook.outgoing_message.data, false, false).unwrap();
        let filters = [
            socketcan::CANFilter::new(
                hook.incomming_message.id,
                CAN_SFF_MASK
            ).unwrap()
        ];
        can_handle.set_filter(&filters).unwrap();
        can_handle.set_read_timeout(std::time::Duration::from_secs(1)).unwrap();
        can_handle.set_nonblocking(true).unwrap();
        let handle = std::thread::Builder::new().name(format!("[PRODUCER THREAD]: {}", self.name)).spawn(move | | {
            loop {
                // Non-blocking read
                if let Ok(_frame) = can_handle.read_frame() {
                    std::thread::sleep(std::time::Duration::from_millis(latency.into()));
                    can_handle.write_frame(&out_message).unwrap();
                }
                match receiver.try_recv() {
                    Ok(WorkerMessage::Stop) => return,
                    Err(std::sync::mpsc::TryRecvError::Disconnected) => return,
                    Err(std::sync::mpsc::TryRecvError::Empty) => {},
                }
            }
        }).unwrap();
        Worker {
            handle,
            sender
        }
    }
}

struct Worker {
    handle: std::thread::JoinHandle<()>,
    sender: std::sync::mpsc::Sender<WorkerMessage>,
}

impl Worker {
    pub fn start(can_interface: &str, device: &impl StartDevice) -> Worker {
        device.start(can_interface)
    }

    pub fn stop(self) {
        self.sender.send(WorkerMessage::Stop).unwrap();
        self.handle.join().unwrap();
    }
}

pub struct INITIALIZATION;
pub struct RUNNING;
pub struct STOPPED;
/**
 * The config Lifetime should be the same as the config object which the producers and the responders are added from
 */
pub struct DeviceManager<'config, State> {
    can_interface: &'config str,
    producers: Vec<&'config config::Producer>,
    responders: Vec<&'config config::Responder>,
    workers: Vec<Worker>,
    state: std::marker::PhantomData<State>
}

impl<'config> DeviceManager<'config, INITIALIZATION> {
    ///
    /// Create a new DeviceManager for a Can Interface. You can use the scripts/start-vcan0.sh script to start the vcan0 interface on your device.
    ///  ```
    /// use virtual_can_device::device_manager::DeviceManager;
    /// let device_manager = DeviceManager::new("vcan0");
    /// ```
    ///
    pub fn new(can_interface: &'config str) -> DeviceManager<'config, INITIALIZATION> {
        DeviceManager {
            can_interface,
            producers: Vec::new(),
            responders: Vec::new(),
            workers: Vec::new(),
            state: std::marker::PhantomData
        }
    }
}

impl<'config>  DeviceManager<'config, INITIALIZATION> {
       ///
    /// ADD A producer to the list of producers
    /// ```
    /// use virtual_can_device::device_manager::DeviceManager;
    /// use virtual_can_device::config::Config;
    /// let mut device_manager = DeviceManager::new("vcan0");
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
    /// let mut device_manager = DeviceManager::new("vcan0");
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
        for producer in self.producers.iter() {
            self.workers.push(Worker::start(self.can_interface, producer))
        }
        for responder in self.responders.iter() {
            self.workers.push(Worker::start(self.can_interface, responder))
        }
        DeviceManager {
            can_interface: self.can_interface,
            producers: self.producers,
            responders: self.responders,
            workers: self.workers,
            state: std::marker::PhantomData
        }
    }
}

impl <'config> DeviceManager<'config, RUNNING> {
    pub fn stop_devices(self) -> DeviceManager<'config, STOPPED>{
        for worker in self.workers.into_iter() {
            worker.stop();
        }
        DeviceManager {
            can_interface: self.can_interface,
            producers: self.producers,
            responders: self.responders,
            workers: Vec::new(),
            state: std::marker::PhantomData
        }
     }
}
