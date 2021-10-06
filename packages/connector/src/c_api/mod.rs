#[derive(Copy, Clone)]
#[repr(C)]
struct am_device {
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

type AMDeviceNotificationCallback = extern "C" fn(_: *mut am_device_notification_callback_info, _: libc::c_int);

#[allow(dead_code)]
extern {
    fn getUDID(device: am_device) -> * const libc::c_char;
    fn AMDeviceNotificationSubscribeBridge(callback: AMDeviceNotificationCallback);
}

fn get_device_udid<'a>(device: am_device) -> &'a str {
    let char_ptr = unsafe { getUDID(device) };
    let c_str = unsafe { std::ffi::CStr::from_ptr(char_ptr) };
    return c_str.to_str().unwrap();
}

#[allow(dead_code)]
extern "C" fn handle_am_device_notification(target: *mut am_device_notification_callback_info, _: libc::c_int) {
    unsafe {
        println!("{}", get_device_udid(*((*target).dev)));
    }
}

pub fn subscribe() {
    unsafe {
        AMDeviceNotificationSubscribeBridge(handle_am_device_notification);
        core_foundation::runloop::CFRunLoopRun();
    }
}
