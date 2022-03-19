#[macro_use]
extern crate napi_derive;

use canter::device::bridge::AMDServiceConnectionRef;
use core_foundation::dictionary::CFDictionaryRef;
use napi::threadsafe_function::{
  ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction, ThreadsafeFunctionCallMode,
};
use napi::{Env, JsFunction, JsObject};
use std::sync::Arc;
use std::thread;

mod cf;

#[napi]
fn get_devices() -> Vec<AMDevice> {
  let devices = canter::device::get_devices(0.25);

  let mut array = Vec::<AMDevice>::new();

  for device in devices {
    array.push(AMDevice::new(device));
  }

  array
}

#[napi]
struct AMDevice {
  device: &'static canter::device::bridge::AMDevice,
}

#[napi]
impl AMDevice {
  pub fn new(device: &'static canter::device::bridge::AMDevice) -> AMDevice {
    AMDevice { device: device }
  }

  #[napi(getter)]
  pub fn udid(&self) -> String {
    canter::device::get_udid(self.device)
  }

  #[napi]
  pub fn connect(&mut self) {
    canter::device::connect(self.device);
  }

  #[napi]
  pub fn disconnect(&mut self) {
    canter::device::disconnect(self.device);
  }

  #[napi]
  pub fn start_service(&mut self, service_name: String) -> AMService {
    let connection_ref = canter::device::start_service(self.device, &service_name);
    AMService::new(connection_ref)
  }
}

#[napi]
pub struct AMService {
  connection_ref: AMDServiceConnectionRef,
}

#[napi]
impl AMService {
  pub fn new(connection_ref: AMDServiceConnectionRef) -> AMService {
    AMService {
      connection_ref: connection_ref,
    }
  }

  #[napi]
  pub fn send(&mut self, env: Env, message: JsObject) {
    canter::device::send_message(self.connection_ref, cf::to_cf_dictionary(message, &env));
  }

  #[napi]
  pub fn register_receive_listener(&mut self, callback: JsFunction) {
    let tsfn: ThreadsafeFunction<CFDictionaryRef, ErrorStrategy::Fatal> = callback
      .create_threadsafe_function(0, |ctx: ThreadSafeCallContext<CFDictionaryRef>| {
        Ok(vec![cf::from_object(&ctx.env, ctx.value)])
      })
      .unwrap();

    let ptr = unsafe { Arc::new(*self.connection_ref) };

    thread::spawn(move || loop {
      let item = Arc::clone(&ptr);
      let message = canter::device::receive_message(Arc::into_raw(item));
      tsfn.call(message, ThreadsafeFunctionCallMode::Blocking);
    });
  }
}
