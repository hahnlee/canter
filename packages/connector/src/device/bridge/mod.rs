use std::collections::HashMap;

#[derive(Copy, Clone)]
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
    extern "C" fn(_: *mut am_device_notification_callback_info, _: *mut libc::c_void);

#[allow(dead_code)]
extern "C" {
    fn getUDID(device: am_device) -> *const libc::c_char;
    fn AMDeviceNotificationSubscribeBridge(
        callback: AMDeviceNotificationCallback,
        manager: *mut libc::c_void,
        timeout: f64,
    );
}

pub fn get_device_udid(device: am_device) -> String {
    let char_ptr = unsafe { getUDID(device) };
    let c_str = unsafe { std::ffi::CStr::from_ptr(char_ptr) };
    return String::from(c_str.to_str().unwrap());
}

#[allow(dead_code)]
extern "C" fn handle_am_device_notification(
    target: *mut am_device_notification_callback_info,
    args: *mut libc::c_void,
) {
    let manager = args as *mut HashMap<String, am_device>;
    unsafe {
        let device = *(*target).dev;
        (*manager).insert(get_device_udid(device), device);
    }
}

pub fn get_device_map(timeout: f64) -> HashMap<String, am_device> {
    let mut state: HashMap<String, am_device> = HashMap::new();
    unsafe {
        let state_ptr: *mut libc::c_void = &mut state as *mut _ as *mut libc::c_void;
        AMDeviceNotificationSubscribeBridge(handle_am_device_notification, state_ptr, timeout);
    }
    return state;
}
