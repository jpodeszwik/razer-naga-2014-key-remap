use uinput::{Error, Device};
use uinput::event::Keyboard::All;
use uinput;

pub fn create() -> Result<Device, String> {
    create_device()
        .map_err(|e| format!("{}", e))
}

fn create_device() -> Result<Device, Error> {
    let device = uinput::open("/dev/uinput")?
        .name("razer-naga-virtual-keyboard")?
        .event(All)?
        .create()?;

    Ok(device)
}
