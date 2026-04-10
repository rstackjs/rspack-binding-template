process.env.RSPACK_BINDING = require('node:path').dirname(
  require.resolve('@rspack-template/binding')
);

const binding = require('@rspack-template/binding');

// Register the plugin `MyBannerPlugin` exported by `crates/binding/src/lib.rs`.
binding.registerMyBannerPlugin();

// Register the plugin `MyBannerLoaderPlugin` exported by `crates/binding/src/lib.rs`.
binding.registerMyBannerLoaderPlugin();

const coreImport = require('@rspack/core');
const core =
  typeof coreImport === 'function' ? coreImport : coreImport.default;

if (typeof core !== 'function') {
  throw new TypeError(
    'Failed to load @rspack/core as a callable export. Please use Node.js 20.19+ or 22.12+.'
  );
}

Object.assign(core, coreImport);

/**
 * Creates a wrapper for the plugin `MyBannerPlugin` exported by `crates/binding/src/lib.rs`.
 *
 * Check out `crates/binding/src/lib.rs` for the original plugin definition.
 * This plugin is used in `examples/use-plugin/build.js`.
 *
 * @example
 * ```js
 * const MyBannerPlugin = require('@rspack-template/core').MyBannerPlugin;
 * ```
 *
 * `createNativePlugin` is a function that creates a wrapper for the plugin.
 *
 * The first argument to `createNativePlugin` is the name of the plugin.
 * The second argument to `createNativePlugin` is a resolver function.
 *
 * Options used to call `new MyBannerPlugin` will be passed as the arguments to the resolver function.
 * The return value of the resolver function will be used to initialize the plugin in `MyBannerPlugin` on the Rust side.
 *
 * For the following code:
 *
 * ```js
 * new MyBannerPlugin('// Hello World')
 * ```
 *
 * The resolver function will be called with `'// Hello World'`.
 *
 */
const MyBannerPlugin = core.experiments.createNativePlugin(
  'MyBannerPlugin',
  function (options) {
    return options;
  }
);

Object.defineProperty(core, 'MyBannerPlugin', {
  value: MyBannerPlugin,
});

/**
 * Creates a wrapper for the plugin `MyBannerLoaderPlugin` exported by `crates/binding/src/lib.rs`.
 *
 * Check out `crates/binding/src/lib.rs` for the original plugin definition.
 * This plugin is used in `examples/use-loader/build.js`.
 *
 * @example
 * ```js
 * const MyBannerLoaderPlugin = require('@rspack-template/core').MyBannerLoaderPlugin;
 * ```
 *
 * `createNativePlugin` is a function that creates a wrapper for the plugin.
 *
 * The first argument to `createNativePlugin` is the name of the plugin.
 * The second argument to `createNativePlugin` is a resolver function.
 *
 * Options used to call `new MyBannerLoaderPlugin` will be passed as the arguments to the resolver function.
 * The return value of the resolver function will be used to initialize the plugin in `MyBannerLoaderPlugin` on the Rust side.
 *
 */
const MyBannerLoaderPlugin = core.experiments.createNativePlugin(
  'MyBannerLoaderPlugin',
  function (options) {
    return options;
  }
);

Object.defineProperty(core, 'MyBannerLoaderPlugin', {
  value: MyBannerLoaderPlugin,
});

module.exports = core;
