#[cfg(test)]
mod tests {
    extern {
        fn subscribeDeviceNotification();
    }

    pub fn subscribe() {
        unsafe {
            subscribeDeviceNotification();
            core_foundation::runloop::CFRunLoopRun();
        }
    }

    #[test]
    fn it_works() {
        subscribe();
    }
}
