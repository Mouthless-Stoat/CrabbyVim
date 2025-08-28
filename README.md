# CrabbyVim

<p align="center">
  <img width="595" height="208" alt="image" src="https://github.com/user-attachments/assets/7d0e253d-67eb-4fee-bd1e-0996bedb83d9" />
</p>

A Neovim config completely written in Rust to be blazingly fast and to proof that everything can and will be rewrite in rust.



<details>
<summary>Screenshot</summary>

<img width="1876" height="962" alt="image" src="https://github.com/user-attachments/assets/764421b7-28c4-4a19-af82-cb4c254827c6" />

<img width="1881" height="968" alt="image" src="https://github.com/user-attachments/assets/1954f39c-9bad-4cdc-ad65-152dba6acd61" />

<img width="1876" height="977" alt="image" src="https://github.com/user-attachments/assets/d47c50ca-a1c2-4d10-b637-910b4597584b" />

<img width="1872" height="980" alt="image" src="https://github.com/user-attachments/assets/cb8bcfad-9ff0-45fa-82c7-124a498b3be4" />

<img width="1878" height="974" alt="image" src="https://github.com/user-attachments/assets/e198322c-fea4-41db-9edd-e29bae08614b" />

<img width="1875" height="968" alt="image" src="https://github.com/user-attachments/assets/82333470-02b3-44e4-8cf6-89007d684bdc" />

<img width="1889" height="972" alt="image" src="https://github.com/user-attachments/assets/e4cd1ce4-74da-46c2-949c-93e3d5a839fb" />
</details>

## Requirements

-   Neovim = 0.11
-   Git (for lazy.nvim)
-   A Nerd Font ([Caskadyia Cove](https://github.com/eliheuer/caskaydia-cove) is used by default)
-   `make` (to build mlua)
-   Rust

## Installation

Find your git config path. This can be done by opening your Neovim and run `:echo stdpath("config")`, this should return the configuration path. You should also backup the current version of you Neovim configs. Then `cd` into the config folder, then you can clone the config using `git`:

```
git clone https://github.com/Mouthless-Stoat/CrabbyVim
```

Then you can also remove the `.git` folder, to track your own commit instead:

```
rm -rf .git
```

Then if you want a faster initial startup run `cargo build --release` to pre-build the config before loading. Then start Neovim with `nvim`, the first startup might be a bit buggy with all the plugins installation and stuff so close and start it a second time.

> [!TIP]
> You should run `:checkhealth` after installation to see if you are missing anything.

## How?

This config is written in Rust with the help of [`nvim-oxi`](https://crates.io/crates/nvim-oxi) and [`mlua`](https://crates.io/crates/mlua) to interface with the Lua API. The entry point of the config is [`src/lib.rs`](src/lib.rust). [`init.lua`](init.lua) is simple a build script to build the config if not found put the compiled binary into the correct location and then load it. The config is finicky on the first launch so it is recommended to launch Neovim for the config to build then quit and relaunch Neovim.

## I want to customize/reconfigure the config. How do I do that?

The main file of interest should be [`src/lib.rs`](src/lib.rs), the entry point of the config where all the configuration are located. The config are separated into several rust module for ease of customization. Notable module are the [`lazy`](src/lazy.rs), [`plugins`](src/plugins.rs) and [`lsp`](src/lsp.rs) module. The `lazy` module define several type needed for [`lazy.nvim`](https://github.com/folke/lazy.nvim), mainly to add and configure new plugins to be use by the config as well as bootstrap `lazy.nvim`. The `plugins` module define all the plugins that are pass to `lazy.nvim` for downloading and configuring. And lastly `lsp` define all the language server to be configure and use. Refer to the built in help file to learn how to add plugins and further configuration `:help CrabbyVim`.

## But why tho?

Some time ago I found out about [CatNvim](https://github.com/rewhile/CatNvim), and thought the idea was intriguing. Later while looking at the [Neovim wiki](https://github.com/neovim/neovim/wiki/Related-projects), I found out about [`nvim-oxi`](https://github.com/noib3/nvim-oxi), the Rust API client for Neovim. The idea was funny enough because "Hur hur, rewrite everything in rust" just for the meme.
