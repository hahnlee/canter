use core_foundation::dictionary::CFDictionaryRef;
use core_foundation::propertylist::CFPropertyListFormat;
use core_foundation::string::CFStringRef;

use libc::{c_char, c_uchar, c_uint, c_void};

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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct amd_service_connection {
    pub unknown: [u8; 16],
    pub socket: u32,
    pub unknown2: u32,
    pub secure_io_context: *mut c_void,
    pub flags: u32,
    pub device_connection_id: u32,
    pub service_name: [c_char; 128],
}

unsafe impl Send for amd_service_connection {}
unsafe impl Sync for amd_service_connection {}

pub type AMDServiceConnectionRef = *const amd_service_connection;

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
    pub fn AMDeviceSecureStartService(
        device: *const am_device,
        service_name: CFStringRef,
        options: CFDictionaryRef,
        service_connection: *const AMDServiceConnectionRef,
    ) -> i32;
    pub fn AMDServiceConnectionSendMessage(
        connection: AMDServiceConnectionRef,
        message: CFDictionaryRef,
        format: CFPropertyListFormat,
    ) -> i32;
    pub fn AMDServiceConnectionReceiveMessage(
        connection: AMDServiceConnectionRef,
        response: *const CFDictionaryRef,
        format: *const CFPropertyListFormat,
        unknown0: *const c_void,
        unknown1: *const c_void,
        unknown2: *const c_void,
    ) -> i32;
}
