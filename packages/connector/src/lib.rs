pub mod device;

#[test]
fn it_works() {
    let mut devices = device::get_devices(0.1);
    println!("{:?}", devices.values());
    let selected = devices.values_mut().nth(0);

    match selected {
        Some(phone) => {
            assert_eq!(phone.connected, false);

            phone.connect();
            assert_eq!(phone.connected, true);

            let service = phone.start_service("com.apple.webinspector");
            assert_eq!(service.started, true);

            phone.disconnect();
            assert_eq!(phone.connected, false);
        }
        None => {}
    }
}
