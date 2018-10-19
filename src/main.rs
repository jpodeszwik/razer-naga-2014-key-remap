extern crate evdev_rs;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate uinput;

use std::env;
use std::thread;
use std::time::Duration;

mod naga;
mod key_map;
mod input_device;
mod event_mapper;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let key_mapper = match args.len() {
        2 => key_map::KeyMapper::read_from_file(args[1].as_str())?,
        1 => key_map::KeyMapper::default(),
        _ => { return Err("Too many arguments".to_string()); }
    };

    let mut device = input_device::create();

    loop {
        let naga = naga::Naga::new();

        match naga {
            Some(dev) => event_mapper::map_events(key_mapper, dev, &mut device),
            None => println!("No device")
        }
        thread::sleep(Duration::from_secs(1))
    }
}
