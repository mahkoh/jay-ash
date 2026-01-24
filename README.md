# jay-ash

[![crates.io](https://img.shields.io/crates/v/jay-ash.svg)](https://crates.io/crates/jay-ash)
[![docs.rs](https://docs.rs/jay-ash/badge.svg)](https://docs.rs/jay-ash)
![MSRV](https://img.shields.io/crates/msrv/jay-ash)

This crate is a fork of [ash](https://github.com/ash-rs/ash). It is based on the
ash 0.38 release but with the vulkan bindings updated to vulkan 1.4.341.

You should never expose any of the jay-ash types in public interfaces. Instead
expose the underlying u64 handles and void pointers that are compatible with any
rust vulkan wrapper.

## Breaking Changes

This crate contains the following breaking changes compared to ash 0.38:

- A small number of breaking changes have been made to the vulkan spec itself,
  such as upgrading extension versions. This is out of our control.
- Constants of the form `KHR_SURFACE_NAME` have been renamed to
  `KHR_SURFACE_EXTENSION_NAME`. The old naming scheme was bespoke. The new
  constant names are taken directly from the vulkan specification. This has no
  effect on your code if you are using the `khr::surface::NAME` aliases.
- The same applies to a lesser extent to constants of the form
  `KHR_SURFACE_SPEC_VERSION`.

## MSRV

The MSRV is `stable - 3`.

## License

This project is licensed under either of

- Apache License, Version 2.0
- MIT License

at your option.
