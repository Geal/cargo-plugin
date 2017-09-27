# cargo-plugin, a tool to handle plugins at compile tome for your crate

**Warning: experimental project, things might change soon**

This tool helps you build a plugin infrastructure for your Rust project.
Highlights of that system:

- the plugin API is versioned, so plugins compatibility is checked
- it only uses stable Rust code
- you can publish your project on [crates.io](http://crates.io) with a default set of dependencies, but allow an override if building locally from source

The [rust-plugin-test project](https://github.com/Geal/rust-plugin-test) is an example of this usage.

## Installation

Launch the following command:

```
cargo install cargo-plugin
```

## How to use

### Setting it up

This will work better if your project uses a workspace:

- the [`plugin-api` crate](https://github.com/Geal/rust-plugin-test/tree/master/plugin-api) defines traits that plugins should implement
  - `cargo-plugin` expects that this crates exposes a `PluginInformation` trait
- the [`plugins` crate](https://github.com/Geal/rust-plugin-test/tree/master/plugins) holds the plugins
  - its `Cargo.toml` file declares a dependency on `plugin-api` by version. This file will be rewritten by `cargo-plugin`
  - the [`metadata.toml` file](https://github.com/Geal/rust-plugin-test/blob/master/plugins/metadata.toml) indicates the plugin API crate's name
  - all plugins are subdirectories of [`plugins/src`](https://github.com/Geal/rust-plugin-test/tree/master/plugins/src):
    - every plugin has a [`metadata.toml` file](https://github.com/Geal/rust-plugin-test/blob/master/plugins/src/english/metadata.toml) declaring its name and its dependencies
    - every plugin, [in its `mod.rs` file](https://github.com/Geal/rust-plugin-test/blob/master/plugins/src/english/mod.rs):
      - imports the traits from the plugin API crate
      - declares a `PLUGIN_METADATA` const element that implements the `PluginInformation` trait
- the [`main` crate](https://github.com/Geal/rust-plugin-test/tree/master/main):
  - declares a dependency to [the plugin API](https://github.com/Geal/rust-plugin-test/blob/master/main/Cargo.toml#L10)
  - declares a dependency to [the plugins crate](https://github.com/Geal/rust-plugin-test/blob/master/main/Cargo.toml#L11), **by version AND by path**

You can now publish your crates, first the plugin API, then the plugins crate, then the main one.

### Usage

Your crates are published with a default set of plugins. If you want to build a version with a different set of plugins, add and remove plugin folders in `plugins/src`, then run `cargo plugin` while in `plugins/`.

The `cargo-plugin` tool will read the metadata from every plugin folder, then edit the `Cargo.toml` and `src/lib.rs` to do the following tasks:

- import crates used by every plugin
- declare all plugins as submodules
- generate a function declaring the plugins at runtime
