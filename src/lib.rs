
pub mod config;
pub mod device_manager;
use config::Config;

pub fn run_devices(device_config: Config) {
    let responders = device_config.get_responders();
    let mut dm = device_manager::DeviceManager::new("vcan0");
    for producer in device_config.get_producers() {
        dm.add_producer(producer);
    }

    let dm = dm.start_devices();
    std::thread::sleep(std::time::Duration::from_secs(10));
    dm.stop_devices();
}
