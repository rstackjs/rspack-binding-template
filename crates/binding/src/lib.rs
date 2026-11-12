mod loader;
mod plugin;

use napi::JsValue;
use napi_derive::napi;
use rspack_binding_builder::{CustomPluginBuilder, register_custom_plugin};

fn register(name: &str, builder: CustomPluginBuilder) -> napi::Result<()> {
  register_custom_plugin(name.to_owned(), builder).map_err(|()| {
    napi::Error::from_reason(format!(
      "Cannot register plugins under the same name: {name}"
    ))
  })
}

// Wrap these plugins with experiments.createNativePlugin on the JS side (see lib/index.js).
#[napi]
pub fn register_my_banner_plugin() -> napi::Result<()> {
  register("MyBannerPlugin", |_env, options| {
    let banner = options.coerce_to_string()?.into_utf8()?.into_owned()?;
    Ok(Box::new(plugin::MyBannerPlugin::new(banner)))
  })
}

#[napi]
pub fn register_my_banner_loader_plugin() -> napi::Result<()> {
  register("MyBannerLoaderPlugin", |_env, _options| {
    Ok(Box::new(loader::MyBannerLoaderPlugin::new()))
  })
}
