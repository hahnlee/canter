#[macro_use]
extern crate napi_derive;

use napi::{CallContext, JsObject, JsUndefined, JsString, Property, Result};

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
  exports.create_named_method("getDevices", get_devices)?;
  Ok(())
}

#[js_function]
fn get_devices(ctx: CallContext) -> Result<JsObject> {
  let devices = canter::device::get_devices(0.25);

  let device_class = ctx.env.define_class(
    "Device",
    device_constructor,
    &vec![
      Property::new(ctx.env, "connect")?.with_method(connect),
      Property::new(ctx.env, "disconnect")?.with_method(disconnect),
      Property::new(ctx.env, "startService")?.with_method(start_service),
    ],
  )?;

  let mut array = ctx.env.create_array_with_length(devices.len())?;

  for (index, device) in devices.into_iter().enumerate() {
    let arguments: Vec<JsUndefined> = vec![];
    let mut instance = device_class.new(&arguments)?;
    instance.set_named_property("udid", ctx.env.create_string(&device.get_udid())?)?;
    ctx.env.wrap(&mut instance, device)?;
    array.set_element(index as u32, instance)?;
  }

  Ok(array)
}

#[js_function(1)]
fn device_constructor(ctx: CallContext) -> Result<JsUndefined> {
  ctx.env.get_undefined()
}

#[js_function(2)]
fn connect(ctx: CallContext) -> Result<JsUndefined> {
  let this = ctx.this_unchecked::<JsObject>();
  let device = ctx.env.unwrap::<canter::device::Device>(&this)?;

  device.connect();

  ctx.env.get_undefined()
}

#[js_function(2)]
fn disconnect(ctx: CallContext) -> Result<JsUndefined> {
  let this = ctx.this_unchecked::<JsObject>();
  let device = ctx.env.unwrap::<canter::device::Device>(&this)?;

  device.disconnect();

  ctx.env.get_undefined()
}

#[js_function(2)]
fn start_service(ctx: CallContext) -> Result<JsObject> {
  let this = ctx.this_unchecked::<JsObject>();
  let device = ctx.env.unwrap::<canter::device::Device>(&this)?;

  let service = device.start_service(ctx.get::<JsString>(0)?.into_utf8()?.as_str()?);

  let service_class = ctx.env.define_class(
    "Service",
    service_constructor,
    &vec![
    ],
  )?;

  let arguments: Vec<JsUndefined> = vec![];
  let mut instance = service_class.new(&arguments)?;
  ctx.env.wrap(&mut instance, service)?;

  Ok(instance)
}

#[js_function(1)]
fn service_constructor(ctx: CallContext) -> Result<JsUndefined> {
  ctx.env.get_undefined()
}
