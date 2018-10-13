use std::thread;
use std::time::Duration;

mod naga;
mod keymap;
mod input_device;
mod event_mapper;


fn main() {
    let mut device = input_device::create();

    loop {
        let naga = naga::Naga::new();

        match naga {
            Some(dev) => event_mapper::map_events(dev, &mut device),
            None => println!("No device")
        }
        thread::sleep(Duration::from_secs(1))
    }
}