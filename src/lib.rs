
mod config;
pub mod device_manager;
use config::Config;

pub fn run_devices(device_config: Config) {
    let producers = device_config.get_producers();
    let responders = device_config.get_responders();

}
