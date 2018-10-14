use uinput::{Device, open};
use uinput::event::Keyboard::All;

pub fn create() -> Device {
    open("/dev/uinput").unwrap()
        .name("razer-naga-virtual-keyboard").unwrap()
        .event(All).unwrap()
        .create().unwrap()
}
