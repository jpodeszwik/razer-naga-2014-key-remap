use evdev_rs::enums::EventCode::{EV_KEY, EV_SYN};
use super::keymap;
use super::naga::Naga;
use uinput::device::Device;

pub fn map_events(naga: Naga, input_device: &mut Device) {
    loop {
        let event = naga.next_event();
        match event {
            Ok(k) => {
                match k.1.event_code {
                    EV_KEY(key) => {
                        let key = keymap::map_key(key).unwrap();
                        match k.1.value {
                            1 => input_device.press(&key).unwrap(),
                            0 => input_device.release(&key).unwrap(),
                            _ => {}
                        }
                    }
                    EV_SYN(_) => input_device.synchronize().unwrap(),
                    _ => {}
                }
            }
            Err(e) => {
                println!("Err: {}", e);
                return;
            }
        }
    }
}
