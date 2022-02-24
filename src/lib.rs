
pub mod config;
pub mod device_manager;
use config::Config;

pub fn run_devices<'config>(device_config: &'config Config, can_interface: &'static str) -> device_manager::DeviceManager<'config, device_manager::RUNNING> {
    let mut dm = device_manager::DeviceManager::new(can_interface);
    for responder in device_config.get_responders() {
        dm.add_responder(responder);
    }
    for producer in device_config.get_producers() {
        dm.add_producer(producer);
    }

    dm.start_devices()
}


pub fn get_configs(directory_path: &str) -> std::io::Result<Vec<String>>  {
    let mut files: Vec<String> = Vec::new();
    for entry in std::fs::read_dir(directory_path)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            if let Some(file) = path.to_str() {
                files.push(String::from(file));
            }
        }
    }
    Ok(files)
}
