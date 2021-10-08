pub mod device;

#[test]
fn it_works() {
    let mut devices = device::get_devices(0.1);
    
    println!("{:?}", devices.values());
    let selected = devices.values_mut().nth(0);

    match selected {
        Some(v) => {
            assert_eq!(v.connected, false);

            v.connect();
            assert_eq!(v.connected, true);

            v.disconnect();
            assert_eq!(v.connected, false);
        }
        None => {},
    }
}
