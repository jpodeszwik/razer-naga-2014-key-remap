use evdev_rs::Device;
use evdev_rs::InputEvent;
use evdev_rs::ReadStatus;
use std::fs;
use std::fs::File;

pub struct Naga {
    device: Device,
    // need to keep this file, otherwise file would be closed too early
    #[allow(dead_code)]
    file: File,
}

impl Naga {
    pub fn new() -> Option<Naga> {
        let paths = fs::read_dir("/dev/input").unwrap();

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

            if dev.name().unwrap().eq("Razer Razer Naga 2014") {
                return Some(Naga { device: dev, file: f });
            }
        }

        None
    }

    pub fn next_event(&self) -> Result<(ReadStatus, InputEvent), String> {
        match self.device.next_event(evdev_rs::NORMAL | evdev_rs::BLOCKING) {
            Ok(res) => Ok(res),
            Err(errno) => Err(format!("{}", errno))
        }
    }
}