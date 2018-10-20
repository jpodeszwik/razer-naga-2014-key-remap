use uinput::{default, Device};
use uinput::Error;
use uinput::event::Keyboard::All;

pub fn create() -> Result<Device, String> {
    create_device()
        .map_err(|e| format!("{}", e))
}

fn create_device() -> Result<Device, Error> {
    let device = default()?
        .name("razer-naga-virtual-keyboard")?
        .event(All)?
        .create()?;

    Ok(device)
}
