mod test {
    use super::*;

}
use socketcan::CANSocket;
use crate::config;

/**
 * The config Lifetime should be the same as the config object which the producers and the responders are added from
 */
pub struct DeviceManager<'config> {
    can_socket_handle: CANSocket,
    producers: Vec<&'config config::Producer>,
    responders: Vec<&'config config::Responder>,
}

impl<'config> DeviceManager<'config> {
    ///
    /// Create a new DeviceManager for a Can Interface. You can use the scripts/start-vcan0.sh script to start the vcan0 interface on your device.
    ///  ```
    /// use virtual_can_device::device_manager::DeviceManager;
    /// let device_manager = DeviceManager::new("vcan0").unwrap();
    /// ```
    ///
    pub fn new(can_interface: &str) -> Result<DeviceManager, Box<dyn std::error::Error>> {
        let can_socket_handle = CANSocket::open(can_interface)?;
        Ok(DeviceManager {
            can_socket_handle,
            producers: Vec::new(),
            responders: Vec::new(),
        })
    }
    pub fn start_devices() { todo!() }
    pub fn stop_devices() { todo!() }
    pub fn add_producer(producer: &'config config::Producer) { todo!() }
    pub fn add_responder(responder: &'config config::Responder) { todo!() }
}
