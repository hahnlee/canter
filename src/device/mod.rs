pub mod bridge;

use core_foundation::base::{Boolean, CFRelease, CFTypeRef, ToVoid};
use core_foundation::dictionary::CFDictionaryRef;
use core_foundation::propertylist::kCFPropertyListBinaryFormat_v1_0;
use core_foundation::runloop::{kCFRunLoopDefaultMode, CFRunLoopRunInMode};
use core_foundation::string::{
    kCFStringEncodingUTF8, CFString, CFStringGetCStringPtr, CFStringRef,
};

extern "C" fn handle_am_device_notification(
    target: *const bridge::am_device_notification_callback_info,
    args: *mut libc::c_void,
) {
    let manager = args as *mut Vec<&bridge::am_device>;
    let device = unsafe { &*(*target).dev };
    unsafe {
        (*manager).push(device);
    }
}

pub fn get_devices(timeout: f64) -> Vec<&'static bridge::am_device> {
    let mut devices = Vec::new();

    unsafe {
        let devices_ptr: *mut libc::c_void = &mut devices as *mut _ as *mut libc::c_void;
        bridge::AMDeviceNotificationSubscribe(handle_am_device_notification, 0, 0, devices_ptr);
        CFRunLoopRunInMode(kCFRunLoopDefaultMode, timeout, false as Boolean);
    }

    return devices;
}

pub fn get_udid(device: &bridge::am_device) -> String {
    let char_ptr = unsafe {
        let ns_uuid = bridge::AMDeviceCopyDeviceIdentifier(device);
        let c_str_ptr = CFStringGetCStringPtr(ns_uuid, kCFStringEncodingUTF8);
        CFRelease(ns_uuid as CFTypeRef);
        c_str_ptr
    };
    let c_str = unsafe { std::ffi::CStr::from_ptr(char_ptr) };
    return String::from(c_str.to_str().unwrap());
}

pub fn pair(device: &bridge::am_device) {
    let is_paired = unsafe { bridge::AMDeviceIsPaired(device) };
    if is_paired != 1 {
        let pair_result = unsafe { bridge::AMDevicePair(device) };
        if pair_result != 0 {
            panic!("device locked");
        }
    }

    let is_valid = unsafe { bridge::AMDeviceValidatePairing(device) };

    if is_valid != 0 {
        panic!("validation failed");
    }
}

pub fn connect(device: &bridge::am_device) {
    let result = unsafe { bridge::AMDeviceConnect(device) };
    if result != 0 {
        panic!("not connected");
    }

    pair(device);

    let session_result = unsafe { bridge::AMDeviceStartSession(device) };
    if session_result != 0 {
        panic!("couldn't start session");
    }
}

pub fn disconnect(device: &bridge::am_device) {
    unsafe {
        bridge::AMDeviceStopSession(device);
        bridge::AMDeviceDisconnect(device);
    };
}

pub fn start_service(
    device: &bridge::am_device,
    service_name: &str,
) -> bridge::AMDServiceConnectionRef {
    unsafe {
        let ns_service_name = CFString::new(&service_name);
        let ns_service_name = ns_service_name.to_void() as CFStringRef;

        let service_ptr: bridge::AMDServiceConnectionRef = std::ptr::null_mut();
        let result = bridge::AMDeviceSecureStartService(
            device,
            ns_service_name,
            std::ptr::null_mut(),
            &service_ptr,
        );

        if result != 0 {
            panic!("couldn't start service {}", result);
        }

        service_ptr
    }
}

pub fn send_message(connection_ref: bridge::AMDServiceConnectionRef, message: CFDictionaryRef) {
    let result = unsafe {
        bridge::AMDServiceConnectionSendMessage(
            connection_ref,
            message,
            kCFPropertyListBinaryFormat_v1_0,
        )
    };

    if result != 0 {
        panic!("couldn't send message {}", result);
    }
}

pub fn receive_message(connection_ref: bridge::AMDServiceConnectionRef) -> CFDictionaryRef {
    unsafe {
        let response: CFDictionaryRef = std::ptr::null_mut();
        let res = bridge::AMDServiceConnectionReceiveMessage(
            connection_ref,
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
