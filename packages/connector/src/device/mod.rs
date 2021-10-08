mod bridge;

use std::collections::HashMap;
use std::fmt;

pub struct Device<'a> {
    am_device: &'a bridge::am_device,
    pub connected: bool,
}

impl Device<'_> {
    pub fn new(am_device: &bridge::am_device) -> Device {
        Device {
            am_device: am_device,
            connected: false,
        }
    }

    pub fn get_udid(&self) -> String {
        bridge::get_device_udid(&self.am_device)
    }

    fn pair(&self) {
        let is_paired = unsafe { bridge::AMDeviceIsPaired(self.am_device) };
        if is_paired != 1 {
            let pair_result = unsafe { bridge::AMDevicePair(self.am_device) };
            if pair_result != 0 {
                panic!("device locked");
            }
        }

        let is_valid = unsafe { bridge::AMDeviceValidatePairing(self.am_device) };

        if is_valid != 0 {
            panic!("validation failed");
        }
    }

    pub fn connect(&mut self) {
        if self.connected {
            return;
        }

        let result = unsafe { bridge::AMDeviceConnect(self.am_device) };
        if result != 0 {
            panic!("not connected");
        }

        self.pair();

        let session_result = unsafe {
            bridge::AMDeviceStartSession(self.am_device)
        };
        if session_result != 0 {
            panic!("couldn't start session");
        }

        self.connected = true;
    }

    pub fn disconnect(&mut self) {
        if !self.connected {
            return;
        }

        unsafe {
            bridge::AMDeviceStopSession(self.am_device);
			bridge::AMDeviceDisconnect(self.am_device);
        };

        self.connected = false;
    }
}

impl fmt::Debug for Device<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Device")
            .field("udid", &self.get_udid())
            .field("connected", &self.connected)
            .finish()
    }
}

pub fn get_devices<'a>(timeout: f64) -> HashMap<String, Device<'a>> {
    let am_devices = bridge::get_device_map(timeout);
    let mut devices = HashMap::new();

    for (uuid, am_device) in am_devices {
        devices.insert(uuid, Device::new(am_device));
    }

    return devices;
}
