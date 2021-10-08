mod bridge;

use std::collections::HashMap;
use std::fmt;

pub struct Device {
    am_device: bridge::am_device,
}

impl Device {
    pub fn new(am_device: bridge::am_device) -> Device {
        Device {
            am_device: am_device,
        }
    }

    pub fn get_udid(&self) -> String {
        bridge::get_device_udid(self.am_device)
    }
}

impl fmt::Debug for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Device")
            .field("udid", &self.get_udid())
            .finish()
    }
}

pub fn get_devices(timeout: f64) -> HashMap<String, Device> {
    let am_devices = bridge::get_device_map(timeout);
    let mut devices = HashMap::new();

    for (uuid, am_device) in am_devices {
        devices.insert(uuid, Device::new(am_device));
    }

    return devices;
}
