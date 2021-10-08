mod c_api;

#[test]
fn it_works() {
    let devices = c_api::get_device_map(0.1);
    println!("{:?}", devices.keys());
}
