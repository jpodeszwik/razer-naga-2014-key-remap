use evdev_rs::{BLOCKING, Device, InputEvent, NORMAL, ReadStatus};
use std::fs::{File, read_dir};

pub struct Naga {
    device: Device,
    // need to keep this file, otherwise file would be closed too early
    #[allow(dead_code)]
    file: File,
}

impl Naga {
    pub fn new() -> Option<Naga> {
        let paths = read_dir("/dev/input").unwrap();

        for path in paths {
            let file = File::open(path.unwrap().path());
            if file.is_err() {
                continue;
            }
            let f = file.unwrap();

            let device = Device::new_from_fd(&f);
            if device.is_err() {
                continue;
            }

            let dev = device.unwrap();

            if dev.name().unwrap().eq("Razer Razer Naga 2014") && dev.phys().unwrap().ends_with("/input2") {
                return Some(Naga { device: dev, file: f });
            }
        }

        None
    }

    pub fn next_event(&self) -> Result<(ReadStatus, InputEvent), String> {
        match self.device.next_event(NORMAL | BLOCKING) {
            Ok(res) => Ok(res),
            Err(errno) => Err(format!("{}", errno))
        }
    }
}
