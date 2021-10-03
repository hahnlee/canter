extern {
    fn subscribeDeviceNotification();
}

pub fn subscribe() {
    unsafe {
        subscribeDeviceNotification();
        core_foundation::runloop::CFRunLoopRun();
    }
}