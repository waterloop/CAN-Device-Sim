use virtual_can_device::run_devices;

fn main() {
    let json = std::fs::read("configs/Firmware-Quinn-Sun Jan 23 2022.json").unwrap();
    let config: virtual_can_device::config::Config = serde_json::from_slice(&json).unwrap();
    run_devices(config);
}
