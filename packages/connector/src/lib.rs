#[macro_use]
extern crate napi_derive;

use napi::{Env, JsObject};

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
  device: canter::device::Device<'static>,
}

#[napi]
impl AMDevice {
  pub fn new(device: canter::device::Device<'static>) -> AMDevice {
    AMDevice { device: device }
  }

  #[napi(getter)]
  pub fn udid(&self) -> String {
    self.device.get_udid()
  }

  #[napi]
  pub fn connect(&mut self) {
    self.device.connect();
  }

  #[napi]
  pub fn disconnect(&mut self) {
    self.device.disconnect();
  }

  #[napi]
  pub fn start_service(&mut self, service_name: String) -> AMService {
    let service = self.device.start_service(&service_name);
    let am_service = AMService::new(service);
    am_service
  }
}

#[napi]
struct AMService {
  service: canter::device::Service,
}

#[napi]
impl AMService {
  pub fn new(service: canter::device::Service) -> AMService {
    AMService { service: service }
  }

  #[napi]
  pub fn send(&mut self, env: Env, message: JsObject) {
    self.service.send(cf::to_cf_dictionary(message, &env));
  }

  #[napi]
  pub fn receive(&mut self, env: Env) -> JsObject {
    let message = cf::from_object(&env, self.service.receive());
    message
  }
}
