# CrabbyVim

A Neovim config completely written in Rust to be blazingly fast and to proof that everything can and will be rewrite in rust.

## How?

This config is written in Rust with the help of [`nvim-oxi`](https://crates.io/crates/nvim-oxi) and [`mlua`](https://crates.io/crates/mlua) to interface with the Lua API. The entry point of the config is [`src/lib.rs`](src/lib.rust). [`init.lua`](init.lua) is simple a build script to build the config if not found put the compiled binary into the correct location and then load it. The config is finicky on the first launch so it is recommended to launch Neovim for the config to build then quit and relaunch Neovim.

## I want to customize/reconfigure the config. How do I do that?

The main file of interest should be [`src/lib.rs`](src/lib.rs), the entry point of the config where all the configuration are located. The config are separated into several rust module for ease of customization. Notable module are the [`lazy`](src/lazy.rs), [`plugins`](src/plugins.rs) and [`lsp`](src/lsp.rs) module. The `lazy` module define several type needed for [`lazy.nvim`](https://github.com/folke/lazy.nvim), mainly to add and configure new plugins to be use by the config as well as bootstrap `lazy.nvim`. The `plugins` module define all the plugins that are pass to `lazy.nvim` for downloading and configuring. And lastly `lsp` define all the language server to be configure and use.

### How do I add a new plugin?

There are 2 ways to add a new plugins. If the configuration is minimal simply include the plugins url in the `plugins` module macro. For example to add [`multicursor.nvim`](https://github.com/jake-stewart/multicursor.nvim), Simply add the url of the plugins below the `---` separation line:

```rust
plugin! {
    snacks with highlights;
    // ...
    devicons;
    ---
    "wakatime/vim-wakatime";
    LazyPlugin::new("FilipHarald/aw-watcher-vim").callback(|_|{Ok(())});
    "jake-stewart/multicursor.nvim"
}
```

For more complex configuration is it recommended to make a new module in the [`plugins`](src/plugins) folder. A `plugins` module need a `fn plugins() -> super::Plugins` define within it. This function need to return a `Result<Vec<LazyPlugin>>` for all the plugins to be added. To add the same `multicursor.nvim` plugins using this method, create a `multicursor.rs` file within the `plugins` directory then populate the content with.

```rust
use super::Plugins;

fn plugins() -> Plugins {
    Ok(vec![
        LazyPlugin::new("jake-stewart/multicursor.nvim")
    ])
}
```

Then simply declare this plugin in the `plugins` module:

```rust
plugin! {
    snacks with highlights;
    // ...
    multicursor;
    devicons;
    ---
    "wakatime/vim-wakatime";
    LazyPlugin::new("FilipHarald/aw-watcher-vim").callback(|_|{Ok(())});
}
```

If the plugins require configuring some highlight groups also add a `fn highlights() -> nvim_oxi::Result<()>` to the plugin's module file that configure the highlight group and change the declaration to include `with highlights`:

```rust
plugin! {
    snacks with highlights;
    // ...
    multicursor with highlights;
    devicons;
    ---
    "wakatime/vim-wakatime";
    LazyPlugin::new("FilipHarald/aw-watcher-vim").callback(|_|{Ok(())});
}
```

## But why tho?

Some time ago I found out about [CatNvim](https://github.com/rewhile/CatNvim), and thought the idea was intriguing. Later while looking at the [Neovim wiki](https://github.com/neovim/neovim/wiki/Related-projects), I found out about [`nvim-oxi`](https://github.com/noib3/nvim-oxi), the Rust API client for Neovim. The idea was funny enough because "Hur hur, rewrite everything in rust" just for the meme.

## Requirements

-   Neovim = 0.11
-   Git (for lazy.nvim)
-   A Nerd Font ([Caskadyia Cove](https://github.com/eliheuer/caskaydia-cove) is used by default)
-   `make` (to build mlua)
-   Rust
