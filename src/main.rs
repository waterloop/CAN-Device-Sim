use virtual_can_device::run_devices;
use virtual_can_device::get_configs;
use text_io::scan;

fn main() {
    let configs = get_configs().unwrap();
    println!("Please Choose a Config:");
    for (i, config) in configs.iter().enumerate() {
        println!("{}. {}", i, config);
    }
    println!("Input the number then press enter");
    let input: usize;
    scan!("{}", input);
    let filename = configs.get(input).unwrap();
    let config =  virtual_can_device::config::Config::from(&filename);
    let dm = run_devices(&config, "vcan0");
    let input2: String;
    scan!("{}", input2);
    dm.stop_devices();
}
