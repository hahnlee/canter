use core_foundation::base::{kCFAllocatorDefault, CFGetTypeID, ToVoid};
use core_foundation::boolean::{kCFBooleanTrue, CFBooleanGetTypeID, CFBooleanRef};
use core_foundation::dictionary::{
    kCFTypeDictionaryKeyCallBacks, kCFTypeDictionaryValueCallBacks, CFDictionaryAddValue,
    CFDictionaryCreateMutable, CFDictionaryGetCount, CFDictionaryGetKeysAndValues,
    CFDictionaryGetTypeID, CFDictionaryRef, CFMutableDictionaryRef,
};
use core_foundation::number::{
    kCFNumberSInt64Type, CFNumberGetType, CFNumberGetTypeID, CFNumberGetValue, CFNumberRef,
};
use core_foundation::string::{
    kCFStringEncodingUTF8, CFString, CFStringGetCStringPtr, CFStringGetTypeID, CFStringRef,
};
use libc::c_void;
use napi::{Env, JsObject, JsString, JsUnknown, ValueType};

pub fn set_cf_dict(dict_ref: CFMutableDictionaryRef, key: *const c_void, unknown: JsUnknown) {
    let object_type = unknown.get_type().unwrap();

    if object_type == ValueType::String {
        let cf_string = CFString::new(
            unknown
                .coerce_to_string()
                .unwrap()
                .into_utf8()
                .unwrap()
                .as_str()
                .unwrap(),
        );
        unsafe {
            CFDictionaryAddValue(dict_ref, key, cf_string.to_void());
        }
        return;
    };

    if object_type == ValueType::Object {
        let new_dict_ref = unsafe {
            CFDictionaryCreateMutable(
                kCFAllocatorDefault,
                0,
                &kCFTypeDictionaryKeyCallBacks,
                &kCFTypeDictionaryValueCallBacks,
            )
        };

        let object = unknown.coerce_to_object().unwrap();
        let properties = object.get_property_names().unwrap();
        let length = properties.get_array_length_unchecked().unwrap();

        for i in 0..length {
            let elem_key = properties.get_element::<JsString>(i).unwrap();
            let value = object
                .get_property::<JsString, JsUnknown>(elem_key)
                .unwrap();
            let ns_key = CFString::new(elem_key.into_utf8().unwrap().as_str().unwrap());
            set_cf_dict(new_dict_ref, ns_key.to_void(), value);
        }

        unsafe {
            CFDictionaryAddValue(dict_ref, key, new_dict_ref as *const c_void);
        }
        return;
    }
}

pub fn to_cf_dictionary(object: JsObject) -> CFMutableDictionaryRef {
    let properties = object.get_property_names().unwrap();
    let length = properties.get_array_length_unchecked().unwrap();

    let dict_ref = unsafe {
        CFDictionaryCreateMutable(
            kCFAllocatorDefault,
            0,
            &kCFTypeDictionaryKeyCallBacks,
            &kCFTypeDictionaryValueCallBacks,
        )
    };

    for i in 0..length {
        let key = properties.get_element::<JsString>(i).unwrap();
        let value = object.get_property::<JsString, JsUnknown>(key).unwrap();
        let ns_key = CFString::new(key.into_utf8().unwrap().as_str().unwrap());
        set_cf_dict(dict_ref, ns_key.to_void(), value);
    }

    dict_ref
}

fn to_string(string_ref: CFStringRef) -> &'static str {
    let char_ptr = unsafe { CFStringGetCStringPtr(string_ref, kCFStringEncodingUTF8) };
    let c_str = unsafe { std::ffi::CStr::from_ptr(char_ptr) };
    return c_str.to_str().unwrap();
}

pub fn from_object(env: &Env, dict_ref: CFDictionaryRef) -> JsObject {
    let length = unsafe { CFDictionaryGetCount(dict_ref) as usize };
    let (keys, values) = get_keys_and_values(dict_ref, length);
    let mut obj = env.create_object().unwrap();

    for i in 0..length {
        let key = to_string(keys[i] as CFStringRef);
        let value = values[i];
        set_obj(env, &mut obj, key, value);
    }

    obj
}

fn set_obj(env: &Env, obj: &mut JsObject, key: &'static str, value: *const c_void) {
    let cf_type = unsafe { CFGetTypeID(value) };

    let js_key = env.create_string(key).unwrap();

    if cf_type == unsafe { CFStringGetTypeID() } {
        let value = to_string(value as CFStringRef);
        let value = env.create_string(value).unwrap();
        obj.set_property(js_key, value).unwrap();
        return;
    }

    if cf_type == unsafe { CFDictionaryGetTypeID() } {
        let value = value as CFDictionaryRef;
        let length = unsafe { CFDictionaryGetCount(value) as usize };
        let (keys, values) = get_keys_and_values(value, length);

        let mut dict = env.create_object().unwrap();

        for i in 0..length {
            let value_key = to_string(keys[i] as CFStringRef);
            set_obj(env, &mut dict, value_key, values[i]);
        }

        obj.set_property(js_key, dict).unwrap();
        return;
    }

    if cf_type == unsafe { CFBooleanGetTypeID() } {
        let value = value as CFBooleanRef;
        let value = unsafe { value == kCFBooleanTrue };
        let value = env.get_boolean(value).unwrap();
        obj.set_property(js_key, value).unwrap();
        return;
    }

    if cf_type == unsafe { CFNumberGetTypeID() } {
        let value = value as CFNumberRef;
        let number_type = unsafe { CFNumberGetType(value) };

        if number_type == kCFNumberSInt64Type {
            let mut num: i64 = 100;
            let number_ptr: *mut i64 = &mut num;
            unsafe {
                CFNumberGetValue(value, kCFNumberSInt64Type, number_ptr as *mut c_void);
            }
            let number = unsafe { *number_ptr };
            obj.set_property(js_key, env.create_int64(number).unwrap())
                .unwrap();
            return;
        }
    }
}

fn get_keys_and_values(
    dict_ref: CFDictionaryRef,
    length: usize,
) -> (Vec<*const c_void>, Vec<*const c_void>) {
    let mut keys = Vec::with_capacity(length);
    let mut values = Vec::with_capacity(length);

    unsafe {
        CFDictionaryGetKeysAndValues(dict_ref, keys.as_mut_ptr(), values.as_mut_ptr());
        keys.set_len(length);
        values.set_len(length);
    }

    (keys, values)
}