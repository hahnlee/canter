pub mod device;

#[test]
fn it_works() {
    let devices = device::get_devices(0.1);
    println!("{:?}", devices.values());
}
