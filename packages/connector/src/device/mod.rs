mod bridge;

use std::collections::HashMap;
use std::fmt;

use core_foundation::base::{Boolean, CFRelease, CFTypeRef};
use core_foundation::runloop::{CFRunLoopRunInMode, kCFRunLoopDefaultMode};
use core_foundation::string::{CFStringGetCStringPtr, kCFStringEncodingUTF8};

fn get_device_udid(device: &bridge::am_device) -> String {
    let char_ptr = unsafe {
       let ns_uuid = bridge::AMDeviceCopyDeviceIdentifier(device);
       let c_str_ptr = CFStringGetCStringPtr(ns_uuid, kCFStringEncodingUTF8);
       CFRelease(ns_uuid as CFTypeRef);
       c_str_ptr
    };
    let c_str = unsafe { std::ffi::CStr::from_ptr(char_ptr) };
    return String::from(c_str.to_str().unwrap());
}

extern "C" fn handle_am_device_notification(
    target: *const bridge::am_device_notification_callback_info,
    args: *mut libc::c_void,
) {
    let manager = args as *mut HashMap<String, &bridge::am_device>;
    let device = unsafe { &*(*target).dev };
    unsafe {
        let udid = get_device_udid(device);
        (*manager).insert(udid, device);
    }
}

fn get_device_map<'a>(timeout: f64) -> HashMap<String, &'a bridge::am_device> {
    let mut state: HashMap<String, &bridge::am_device> = HashMap::new();
    unsafe {
        let state_ptr: *mut libc::c_void = &mut state as *mut _ as *mut libc::c_void;
        bridge::AMDeviceNotificationSubscribe(handle_am_device_notification, 0, 0, state_ptr);
        CFRunLoopRunInMode(kCFRunLoopDefaultMode, timeout, false as Boolean);
    }
    return state;
}

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
        get_device_udid(&self.am_device)
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

        let session_result = unsafe { bridge::AMDeviceStartSession(self.am_device) };
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
    let am_devices = get_device_map(timeout);
    let mut devices = HashMap::new();

    for (uuid, am_device) in am_devices {
        devices.insert(uuid, Device::new(am_device));
    }

    return devices;
}
