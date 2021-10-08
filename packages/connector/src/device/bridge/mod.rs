use std::collections::HashMap;

#[repr(C)]
pub struct am_device {
    pub unknown0: [libc::c_uchar; 16],
    pub device_id: libc::c_uint,
    pub product_id: libc::c_uint,
    pub serial: *mut libc::c_char,
    pub unknown1: libc::c_uint,
    pub unknown2: libc::c_uint,
    pub lockdown_conn: libc::c_uint,
    pub unknown3: [libc::c_uchar; 8],
    pub unknown4: libc::c_uint,
    pub unknown5: [libc::c_uchar; 24],
}

#[derive(Copy, Clone)]
#[repr(C)]
struct am_device_notification_callback_info {
    pub dev: *mut am_device,
    pub msg: libc::c_uint,
    pub subscription: *mut am_device_notification,
}

#[derive(Copy, Clone)]
#[repr(C)]
struct am_device_notification {
    pub unknown0: libc::c_uint,
    pub unknown1: libc::c_uint,
    pub unknown2: libc::c_uint,
    pub callback: AMDeviceNotificationCallback,
    pub cookie: libc::c_uint,
}

type AMDeviceNotificationCallback =
    extern "C" fn(_: *const am_device_notification_callback_info, _: *mut libc::c_void);

extern "C" {
    fn getUDID(device: *const am_device) -> *const libc::c_char;
    fn AMDeviceNotificationSubscribeBridge(
        callback: AMDeviceNotificationCallback,
        manager: *mut libc::c_void,
        timeout: f64,
    );
    pub fn AMDeviceConnect(device: *const am_device) -> i32;
    pub fn AMDeviceIsPaired(device: *const am_device) -> i32;
    pub fn AMDevicePair(device: *const am_device) -> i32;
    pub fn AMDeviceValidatePairing(device: *const am_device) -> i32;
    pub fn AMDeviceStartSession(device: *const am_device) -> i32;
    pub fn AMDeviceStopSession(device: *const am_device) -> i32;
    pub fn AMDeviceDisconnect(device: *const am_device) -> i32;
}

pub fn get_device_udid(device: &am_device) -> String {
    let char_ptr = unsafe { getUDID(device as *const am_device) };
    let c_str = unsafe { std::ffi::CStr::from_ptr(char_ptr) };
    return String::from(c_str.to_str().unwrap());
}

extern "C" fn handle_am_device_notification(
    target: *const am_device_notification_callback_info,
    args: *mut libc::c_void,
) {
    let manager = args as *mut HashMap<String, &am_device>;
    let device = unsafe {
        &*(*target).dev
    };
    unsafe {
        let udid = get_device_udid(device);
        (*manager).insert(udid, device);
    }
}

pub fn get_device_map<'a>(timeout: f64) -> HashMap<String, &'a am_device> {
    let mut state: HashMap<String, &am_device> = HashMap::new();
    unsafe {
        let state_ptr: *mut libc::c_void = &mut state as *mut _ as *mut libc::c_void;
        AMDeviceNotificationSubscribeBridge(handle_am_device_notification, state_ptr, timeout);
    }
    return state;
}
