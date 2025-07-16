# Mouthless Nvim Config

Config completely written in Rust to be blazingly fast and to proof that everything can and will be rewrite in rust.

This config is written in Rust with the help of [`nvim-oxi`](https://crates.io/crates/nvim-oxi) and [`mlua`](https://crates.io/crates/mlua) ti interface with the lua api. To customize, the entry point of the config is in [`src/lib.rs`](src/lib.rs), [`init.lua`](init.lua) should never be modified because it simply a bootstrapper to compile the config and load it.

## But why tho?

Some time ago I found out about [CatNvim](https://github.com/rewhile/CatNvim), and thought the idea was intriguing. Later while looking at the [Neovim wiki](https://github.com/neovim/neovim/wiki/Related-projects), I found out about [`nvim-oxi`](https://github.com/noib3/nvim-oxi), the Rust api client for Neovim. The idea was funny enough because "Hur hur, rewrite everything in rust" just for the meme.

## Requirements
- Neovim = 0.11
- Git (for lazy.nvim)
- [Caskadyia Cove](https://github.com/eliheuer/caskaydia-cove) (font of choice)
- [`just`](https://just.systems) (to build rust config)
- Rust
