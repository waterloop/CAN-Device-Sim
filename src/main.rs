use virtual_can_device::run_devices;
use virtual_can_device::get_configs;
use text_io::scan;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long, default_value = "vcan0")]
    can_interface: String,
    #[clap(long, default_value = "./configs")]
    config_folder: String,
}
fn main() {
    let args = Args::parse();
    let configs = get_configs(&args.config_folder).unwrap();
    println!("Please Choose a Config:");
    for (i, config) in configs.iter().enumerate() {
        println!("{}. {}", i, config);
    }
    println!("Input the number then press enter");
    let input: usize;
    scan!("{}", input);
    let filename = configs.get(input).unwrap();
    let config =  virtual_can_device::config::Config::from(&filename);
    let mem_leaker = Box::new(args.can_interface);
    let can_interface: &'static str = Box::leak(mem_leaker);
    let dm = run_devices(&config, can_interface);
    let input2: String;
    scan!("{}", input2);
    dm.stop_devices();
}
