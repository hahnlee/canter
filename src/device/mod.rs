mod bridge;

use std::fmt;

use core_foundation::base::{Boolean, CFRelease, CFTypeRef, ToVoid};
use core_foundation::dictionary::CFDictionaryRef;
use core_foundation::propertylist::kCFPropertyListBinaryFormat_v1_0;
use core_foundation::runloop::{kCFRunLoopDefaultMode, CFRunLoopRunInMode};
use core_foundation::string::{
    kCFStringEncodingUTF8, CFString, CFStringGetCStringPtr, CFStringRef,
};

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
    let manager = args as *mut Vec<Device>;
    let device = unsafe { &*(*target).dev };
    unsafe {
        (*manager).push(Device::new(device));
    }
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

    // TODO: use session trait
    pub fn start_service(&self, service_name: &str) -> Service {
        if !self.connected {
            panic!("device not connected");
        }

        return Service::new(&self.am_device, &service_name);
    }
}

pub struct Service {
    service_ptr: bridge::AMDServiceConnectionRef,
}

impl Service {
    pub fn new(am_device: &bridge::am_device, service_name: &str) -> Service {
        unsafe {
            let ns_service_name = CFString::new(&service_name);
            let ns_service_name = ns_service_name.to_void() as CFStringRef;

            let service_ptr: bridge::AMDServiceConnectionRef = std::ptr::null_mut();

            let result = bridge::AMDeviceSecureStartService(
                am_device,
                ns_service_name,
                std::ptr::null_mut(),
                &service_ptr,
            );

            if result != 0 {
                panic!("couldn't start service {}", result);
            }

            return Service {
                service_ptr: service_ptr,
            };
        }
    }

    pub fn send(&self, message: CFDictionaryRef) {
        let result = unsafe {
            bridge::AMDServiceConnectionSendMessage(
                self.service_ptr,
                message,
                kCFPropertyListBinaryFormat_v1_0,
            )
        };

        if result != 0 {
            panic!("couldn't send message {}", result);
        }
    }

    pub fn receive(&self) -> CFDictionaryRef {
        unsafe {
            let response: CFDictionaryRef = std::ptr::null_mut();
            let res = bridge::AMDServiceConnectionReceiveMessage(
                self.service_ptr,
                &response,
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
            );

            if res != 0 {
                panic!("couldn't receive response");
            }

            response
        }
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

pub fn get_devices(timeout: f64) -> Vec<Device<'static>> {
    let mut devices = Vec::new();

    unsafe {
        let devices_ptr: *mut libc::c_void = &mut devices as *mut _ as *mut libc::c_void;
        bridge::AMDeviceNotificationSubscribe(handle_am_device_notification, 0, 0, devices_ptr);
        CFRunLoopRunInMode(kCFRunLoopDefaultMode, timeout, false as Boolean);
    }

    return devices;
}
