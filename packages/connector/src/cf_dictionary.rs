use libc::c_void;
use std::ops::Drop;

use core_foundation::base::{kCFAllocatorDefault, CFRelease, CFTypeRef, ToVoid};
use core_foundation::dictionary::{
    kCFTypeDictionaryKeyCallBacks, kCFTypeDictionaryValueCallBacks, CFDictionaryAddValue,
    CFDictionaryCreateMutable, CFMutableDictionaryRef,
};
use core_foundation::string::CFString;

pub struct CFDictionary {
    dict_ref: CFMutableDictionaryRef,
}

impl CFDictionary {
    pub fn new() -> CFDictionary {
        let dict_ref = unsafe {
            CFDictionaryCreateMutable(
                kCFAllocatorDefault,
                0,
                &kCFTypeDictionaryKeyCallBacks,
                &kCFTypeDictionaryValueCallBacks,
            )
        };
        CFDictionary { dict_ref: dict_ref }
    }

    pub fn insert(&self, key: &str, value: *const c_void) {
        let ns_key = CFString::new(key);
        let ns_key_ref = ns_key.to_void();
        unsafe {
            CFDictionaryAddValue(self.dict_ref, ns_key_ref, value);
        }
    }
}

unsafe impl ToVoid<*const c_void> for CFDictionary {
    fn to_void(&self) -> *const c_void {
        self.dict_ref as *const c_void
    }
}

impl Drop for CFDictionary {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.dict_ref as CFTypeRef);
        }
    }
}
