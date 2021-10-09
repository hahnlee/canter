use core_foundation::string::CFStringRef;
use libc::{c_char,c_uint,c_uchar,c_void};

#[repr(C)]
pub struct am_device {
    pub unknown0: [c_uchar; 16],
    pub device_id: c_uint,
    pub product_id: c_uint,
    pub serial: *mut c_char,
    pub unknown1: c_uint,
    pub unknown2: c_uint,
    pub lockdown_conn: c_uint,
    pub unknown3: [c_uchar; 8],
    pub unknown4: c_uint,
    pub unknown5: [c_uchar; 24],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct am_device_notification_callback_info {
    pub dev: *mut am_device,
    pub msg: c_uint,
    pub subscription: *mut am_device_notification,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct am_device_notification {
    pub unknown0: c_uint,
    pub unknown1: c_uint,
    pub unknown2: c_uint,
    pub callback: AMDeviceNotificationCallback,
    pub cookie: c_uint,
}

type AMDeviceNotificationCallback =
    extern "C" fn(_: *const am_device_notification_callback_info, _: *mut c_void);

extern "C" {
    pub fn AMDeviceNotificationSubscribe(
        callback: AMDeviceNotificationCallback,
        unknown0: i32,
        unknown2: i32,
        manager: *mut c_void,
    );
    pub fn AMDeviceCopyDeviceIdentifier(device: *const am_device) -> CFStringRef;
    pub fn AMDeviceConnect(device: *const am_device) -> i32;
    pub fn AMDeviceIsPaired(device: *const am_device) -> i32;
    pub fn AMDevicePair(device: *const am_device) -> i32;
    pub fn AMDeviceValidatePairing(device: *const am_device) -> i32;
    pub fn AMDeviceStartSession(device: *const am_device) -> i32;
    pub fn AMDeviceStopSession(device: *const am_device) -> i32;
    pub fn AMDeviceDisconnect(device: *const am_device) -> i32;
}
