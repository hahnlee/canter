pub mod device;

#[cfg(test)]
mod cf_dictionary;

#[cfg(test)]
use core_foundation::base::{CFShow, ToVoid};
#[cfg(test)]
use core_foundation::dictionary::CFDictionaryRef;
#[cfg(test)]
use core_foundation::string::CFString;
#[cfg(test)]
use uuid::Uuid;

#[test]
fn it_works() {
    println!("start test");

    let mut devices = device::get_devices(0.1);
    println!("{:?}", devices);

    let phone = &mut devices[0];

    assert_eq!(phone.connected, false);

    phone.connect();
    assert_eq!(phone.connected, true);

    let service = phone.start_service("com.apple.webinspector");
    let connection_id = Uuid::new_v4().to_string();

    {
        let dict = cf_dictionary::CFDictionary::new();
        let args = cf_dictionary::CFDictionary::new();

        args.insert(
            "WIRConnectionIdentifierKey",
            CFString::new(&connection_id).to_void(),
        );
        dict.insert(
            "__selector",
            CFString::new("_rpc_reportIdentifier:").to_void(),
        );
        dict.insert("__argument", args.to_void());
        service.send(dict.to_void() as CFDictionaryRef);
        let res = service.receive();
        unsafe {
            CFShow(res as *mut libc::c_void);
        }
    }

    {
        let dict = cf_dictionary::CFDictionary::new();
        let args = cf_dictionary::CFDictionary::new();
        args.insert(
            "WIRConnectionIdentifierKey",
            CFString::new(&connection_id).to_void(),
        );
        dict.insert(
            "__selector",
            CFString::new("_rpc_getConnectedApplications:").to_void(),
        );
        dict.insert("__argument", args.to_void());
        service.send(dict.to_void() as CFDictionaryRef);
        let res = service.receive();
        unsafe {
            CFShow(res as *mut libc::c_void);
        }
    }

    phone.disconnect();
    assert_eq!(phone.connected, false);
}
