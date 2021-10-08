mod bridge;

#[test]
fn it_works() {
    let devices = bridge::get_device_map(0.1);
    println!("{:?}", devices.keys());
}
