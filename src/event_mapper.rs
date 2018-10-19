use evdev_rs::enums::EventCode::{EV_KEY, EV_SYN};
use evdev_rs::InputEvent;
use super::key_map::KeyMapper;
use super::naga::Naga;
use uinput::device::Device;

pub fn map_events(key_mapper: KeyMapper, naga: Naga, input_device: &mut Device) {
    loop {
        let event = naga.next_event();
        match event {
            Ok((_read_status, input_event)) => process_event(key_mapper, input_event, input_device),
            Err(e) => {
                println!("Err: {}", e);
                return;
            }
        }
    }
}

fn process_event(key_mapper: KeyMapper, event: InputEvent, input_device: &mut Device) {
    match event.event_code {
        EV_KEY(key) => {
            let mapped_key = key_mapper.map_key(key).unwrap();
            match event.value {
                1 => input_device.press(&mapped_key).unwrap(),
                0 => input_device.release(&mapped_key).unwrap(),
                _ => {}
            }
        }
        EV_SYN(_) => input_device.synchronize().unwrap(),
        _ => {}
    }
}
