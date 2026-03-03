# Rspack binding template

**🚀 Unlock native Rust speed for Rspack — supercharge your builds, keep every JS feature, zero compromise, no limits.**

## Features

- 🦀 Write your own Rspack plugins and loaders in Rust
- 🧩 Inherit all Rspack features and JavaScript API
- 🛡️ Secure supply chain with npm provenance
- 📦 Effortless publishing: just set your `NPM_TOKEN`

## Quick Start

📖 **[Create custom binding](https://rstackjs.github.io/rspack-rust-book/custom-binding/getting-started/index.html)**

## Why?

Rspack achieves high performance by being written in Rust, but using its JavaScript API introduces overhead due to cross-language calls. This can limit performance and access to native Rust features.

_Rspack Custom Binding_ allows you to extend Rspack directly with native Rust code, avoiding the JavaScript layer and unlocking full performance and flexibility.

With custom binding, you can still use the familiar JavaScript API (`@rspack/core`), but your custom logic runs natively, combining the best of both worlds.

Check out [rationale](https://rstackjs.github.io/rspack-rust-book/custom-binding/getting-started/rationale.html) for more details.

## Supported Platforms

| Target                        | Host Runner    | Notes               |
| ----------------------------- | -------------- | ------------------- |
| x86_64-apple-darwin           | macos-latest   | macOS Intel         |
| aarch64-apple-darwin          | macos-latest   | macOS Apple Silicon |
| x86_64-pc-windows-msvc        | windows-latest | Windows 64-bit      |
| i686-pc-windows-msvc          | windows-latest | Windows 32-bit      |
| aarch64-pc-windows-msvc       | windows-latest | Windows ARM64       |
| x86_64-unknown-linux-gnu      | ubuntu-22.04   | Linux x64 (GNU)     |
| x86_64-unknown-linux-musl     | ubuntu-22.04   | Linux x64 (musl)    |
| aarch64-unknown-linux-gnu     | ubuntu-22.04   | Linux ARM64 (GNU)   |
| aarch64-unknown-linux-musl    | ubuntu-22.04   | Linux ARM64 (musl)  |
| armv7-unknown-linux-gnueabihf | ubuntu-22.04   | Linux ARMv7         |
| aarch64-linux-android         | ubuntu-22.04   | Android ARM64       |
| armv7-linux-androideabi       | ubuntu-22.04   | Android ARMv7       |

> **Note:** Node.js support requires >= 18.
>
> Multi-platform publishing and CI support is powered by [rspack-toolchain](https://github.com/rstackjs/rspack-toolchain). For the latest supported platforms, see the [official supported targets list](https://github.com/rstackjs/rspack-toolchain/tree/main?tab=readme-ov-file#supported-targets).
