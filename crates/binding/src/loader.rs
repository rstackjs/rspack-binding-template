use std::sync::Arc;

use rspack_core::{NormalModuleFactoryResolveLoader, RunnerContext};
use rspack_error::Result;
use rspack_hook::{plugin, plugin_hook};

/// Prepend a banner to the source code
#[rspack_cacheable::cacheable]
pub struct MyBannerLoader {
  banner: String,
}

impl MyBannerLoader {
  pub fn new(banner: String) -> Self {
    Self { banner }
  }
}

#[rspack_cacheable::cacheable_dyn]
#[async_trait::async_trait]
impl rspack_core::Loader<RunnerContext> for MyBannerLoader {
  fn identifier(&self) -> rspack_collections::Identifier {
    rspack_collections::Identifier::from("builtin:my-banner-loader")
  }

  async fn run(
    &self,
    loader_context: &mut rspack_core::LoaderContext<RunnerContext>,
  ) -> rspack_error::Result<()> {
    let source = loader_context.take_content();

    if let Some(source) = source {
      let source = source.try_into_string()?;
      let source = format!("{}\n{}", self.banner, source);
      loader_context.finish_with(source);
    } else {
      loader_context.finish_with_empty();
    }
    Ok(())
  }
}

/// A plugin that provides the `builtin:my-banner-loader` loader to Rspack
#[plugin]
#[derive(Debug)]
pub struct MyBannerLoaderPlugin;

impl MyBannerLoaderPlugin {
  pub fn new() -> Self {
    Self::new_inner()
  }
}

/// Resolves the `builtin:my-banner-loader` loader
#[plugin_hook(NormalModuleFactoryResolveLoader for MyBannerLoaderPlugin, tracing = false)]
pub(crate) async fn resolve_loader(
  &self,
  _context: &rspack_core::Context,
  _resolver: &rspack_core::Resolver,
  loader: &rspack_core::ModuleRuleUseLoader,
  _data: &mut rspack_core::ModuleFactoryCreateData,
) -> Result<Option<rspack_core::BoxLoader>> {
  if loader.loader.starts_with("builtin:my-banner-loader") {
    let banner = loader.options.clone().unwrap_or_default();
    return Ok(Some(Arc::new(MyBannerLoader::new(banner))));
  }

  Ok(None)
}

impl rspack_core::Plugin for MyBannerLoaderPlugin {
  fn apply(&self, ctx: &mut rspack_core::ApplyContext) -> Result<()> {
    ctx
      .normal_module_factory_hooks
      .resolve_loader
      .tap(resolve_loader::new(self));
    Ok(())
  }
}
