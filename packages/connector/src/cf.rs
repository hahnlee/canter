use core_foundation::base::{
    kCFAllocatorDefault, Boolean, CFGetTypeID, CFIndex, CFRange, ToVoid,
};
use core_foundation::boolean::{kCFBooleanTrue, CFBooleanGetTypeID, CFBooleanRef};
use core_foundation::data::{
    CFDataCreate, CFDataGetBytePtr, CFDataGetLength, CFDataGetTypeID, CFDataRef,
};
use core_foundation::dictionary::{
    kCFTypeDictionaryKeyCallBacks, kCFTypeDictionaryValueCallBacks, CFDictionaryAddValue,
    CFDictionaryCreateMutable, CFDictionaryGetCount, CFDictionaryGetKeysAndValues,
    CFDictionaryGetTypeID, CFDictionaryRef, CFMutableDictionaryRef,
};
use core_foundation::number::{
    kCFNumberSInt64Type, CFNumber, CFNumberGetType, CFNumberGetTypeID, CFNumberGetValue,
    CFNumberRef,
};
use core_foundation::string::{
    kCFStringEncodingUTF8, CFString, CFStringGetBytes, CFStringGetCStringPtr, CFStringGetLength,
    CFStringGetTypeID, CFStringRef,
};
use libc::c_void;
use napi::{Env, JsBoolean, JsFunction, JsNumber, JsObject, JsString, JsUnknown, ValueType};

pub fn set_cf_dict(
    dict_ref: CFMutableDictionaryRef,
    key: *const c_void,
    unknown: JsUnknown,
    env: &Env,
) {
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
        if unknown.is_typedarray().unwrap() {
            let object = unknown.coerce_to_object().unwrap();
            let properties = object.get_property_names().unwrap();
            let length = properties.get_array_length_unchecked().unwrap();
            let mut data: Vec<u8> = vec![];
            for i in 0..length {
                let elem_value = object.get_element::<JsNumber>(i).unwrap();
                let value = (elem_value.get_uint32().unwrap() & 0xff) as u8;
                data.push(value);
            }

            let cf_data = unsafe {
                CFDataCreate(kCFAllocatorDefault, data.as_mut_ptr(), data.len() as isize)
            };

            unsafe {
                CFDictionaryAddValue(dict_ref, key, cf_data.to_void());
            }

            return;
        }

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
            set_cf_dict(new_dict_ref, ns_key.to_void(), value, env);
        }

        unsafe {
            CFDictionaryAddValue(dict_ref, key, new_dict_ref as *const c_void);
        }
        return;
    }

    if object_type == ValueType::Number {
        let value = unknown.coerce_to_number().unwrap();
        let cf_number = to_cf_number(value, env);

        unsafe {
            CFDictionaryAddValue(dict_ref, key, cf_number.to_void());
        }
    }
}

fn to_cf_number(value: JsNumber, env: &Env) -> CFNumber {
    // TODO: do better way
    let number_object = env
        .get_global()
        .unwrap()
        .get_property::<JsString, JsFunction>(env.create_string("Number").unwrap())
        .unwrap()
        .coerce_to_object()
        .unwrap();

    let is_integer_fn = number_object
        .get_property::<JsString, JsFunction>(env.create_string("isInteger").unwrap())
        .unwrap();

    let is_integer = unsafe {
        is_integer_fn
            .call(None, &[value])
            .unwrap()
            .cast::<JsBoolean>()
            .get_value()
            .unwrap()
    };

    if is_integer {
        CFNumber::from(value.get_int32().unwrap())
    } else {
        CFNumber::from(value.get_double().unwrap())
    }
}

pub fn to_cf_dictionary(object: JsObject, env: &Env) -> CFMutableDictionaryRef {
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
        set_cf_dict(dict_ref, ns_key.to_void(), value, env);
    }

    dict_ref
}

fn to_string(string_ref: CFStringRef) -> String {
    // reference: https://github.com/servo/core-foundation-rs/blob/355740/core-foundation/src/string.rs#L49
    unsafe {
        let char_ptr = CFStringGetCStringPtr(string_ref, kCFStringEncodingUTF8);
        if !char_ptr.is_null() {
            let c_str = std::ffi::CStr::from_ptr(char_ptr);
            return String::from(c_str.to_str().unwrap());
        }

        let char_len = CFStringGetLength(string_ref);

        let mut bytes_required: CFIndex = 0;
        CFStringGetBytes(
            string_ref,
            CFRange {
                location: 0,
                length: char_len,
            },
            kCFStringEncodingUTF8,
            0,
            false as Boolean,
            std::ptr::null_mut(),
            0,
            &mut bytes_required,
        );

        // Then, allocate the buffer and actually copy.
        let mut buffer = vec![b'\x00'; bytes_required as usize];

        let mut bytes_used: CFIndex = 0;
        CFStringGetBytes(
            string_ref,
            CFRange {
                location: 0,
                length: char_len,
            },
            kCFStringEncodingUTF8,
            0,
            false as Boolean,
            buffer.as_mut_ptr(),
            buffer.len() as CFIndex,
            &mut bytes_used,
        );

        return String::from_utf8_unchecked(buffer);
    }
}

pub fn from_object(env: &Env, dict_ref: CFDictionaryRef) -> JsObject {
    let length = unsafe { CFDictionaryGetCount(dict_ref) as usize };
    let (keys, values) = get_keys_and_values(dict_ref, length);
    let mut obj = env.create_object().unwrap();

    for i in 0..length {
        let key = to_string(keys[i] as CFStringRef);
        let value = values[i];
        set_obj(env, &mut obj, &key, value);
    }

    obj
}

fn set_obj(env: &Env, obj: &mut JsObject, key: &str, value: *const c_void) {
    let cf_type = unsafe { CFGetTypeID(value) };

    let js_key = env.create_string(key).unwrap();

    if cf_type == unsafe { CFStringGetTypeID() } {
        let value = to_string(value as CFStringRef);
        let value = env.create_string(&value).unwrap();
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
            set_obj(env, &mut dict, &value_key, values[i]);
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

    if cf_type == unsafe { CFDataGetTypeID() } {
        let value = value as CFDataRef;
        let ptr = unsafe { CFDataGetBytePtr(value) };
        let length = unsafe { CFDataGetLength(value) };
        let slice = unsafe { std::slice::from_raw_parts(ptr, length as usize) };
        obj.set_property(
            js_key,
            env.create_arraybuffer_with_data(slice.to_vec())
                .unwrap()
                .into_raw(),
        )
        .unwrap()
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
