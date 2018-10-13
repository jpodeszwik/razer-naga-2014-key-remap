use uinput::Device;

pub fn create() -> Device {
    uinput::open("/dev/uinput").unwrap()
        .name("razer-naga-virtual-keyboard").unwrap()
        .event(uinput::event::Keyboard::All).unwrap()
        .create().unwrap()
}
