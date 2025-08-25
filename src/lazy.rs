//! Configure and interface with [`lazy.nvim`](https://github.com/folke/lazy.nvim).
//!
//! Configuring is done using the main [`Lazy`] struct.
//!
//! # Examples
//! ```rust
//! use crate::lazy::*;
//! use crate::{lua_table, table};
//!
//! let mut lazty = Lazy::new();
//! lazy.add_plugin("nvim-tree/nvim-web-devicons")
//! lazy.add_plugin(
//!     lazyplugin::new("stevearc/conform.nvim")
//!         .opts(table! {
//!             formatters_by_ft = table! {
//!                 lua = ["stylua"],
//!                 python = ["black"],
//!                 javascript = ["prettier"],
//!                 typescript = ["prettier"],
//!                 json = ["prettier"],
//!                 markdown = ["prettier"],
//!                 rust = ["rustfmt"],
//!                 yaml = ["prettier"],
//!                 toml = ["taplo"]
//!             },
//!             format_on_save = table! {
//!                 timeout_ms = 5000,
//!                 lsp_format = "fallback"
//!             }
//!         })
//!         .lazy_load(lazyload::new(true).events(&["BufWritePre"]))
//! );
//! lazy.add_plugin(
//!     LazyPlugin::new("folke/snacks.nvim")
//!         .depend(&[
//!             "nvim-tree/nvim-web-devicons",
//!             "aznhe21/actions-preview.nvim",
//!         ])
//!         .opts(table! {
//!             indent = table! {},
//!             picker = table! {},
//!             lazygit = table! {}
//!         })
//!         .lazy_load(
//!             LazyLoad::new(false)
//!                 .add_key(LazyKey::new("<Leader>g").action(|| {
//!                     require("snacks")?
//!                         .get::<Table>("lazygit")?
//!                         .call_function::<()>("open", ())?;
//!                     Ok(())
//!                 }))
//!                 .add_keys(vec![
//!                     LazyKey::new("<Leader>st").action(picker("files", table! {})),
//!                     LazyKey::new("<Leader>st").action(picker("grep", table! {})),
//!                     LazyKey::new("<Leader>sr").action(picker("recent", table! {})),
//!                     LazyKey::new("<Leader>ss").action(picker("lsp_workspace_symbols", table! {})),
//!                     LazyKey::new("<Leader>sS").action(picker("lsp_symbols", table! {})),
//!                 ])
//!         )
//! )
//! lazy.setup()?
//! ```

use mlua::IntoLua;

use crate::Mode;
use crate::{keymaps::Action, table, vim_fn};

/// Main struct for configuring and setting up lazy.
///
/// Creating a new instance with [`Lazy::new`] then add all the plugins require using
/// [`Lazy::add_plugin`] and [`Lazy::add_plugins`] then finally call [`Lazy::setup`] to bootstrap
/// and configure lazy
///
/// # Examples
/// ```rust
/// let mut lazty = lazy::new();
/// lazy.add_plugin("nvim-tree/nvim-web-devicons")
/// lazy.add_plugin(
///     lazyplugin::new("stevearc/conform.nvim")
///         .opts(table! {
///             formatters_by_ft = table! {
///                 lua = ["stylua"],
///                 python = ["black"],
///                 javascript = ["prettier"],
///                 typescript = ["prettier"],
///                 json = ["prettier"],
///                 markdown = ["prettier"],
///                 rust = ["rustfmt"],
///                 yaml = ["prettier"],
///                 toml = ["taplo"]
///             },
///             format_on_save = table! {
///                 timeout_ms = 5000,
///                 lsp_format = "fallback"
///             }
///         })
///         .lazy_load(lazyload::new(true).events(&["bufwritepre"]))
/// )  
/// lazy.setup()?
/// ```
pub struct Lazy(Vec<LazyPlugin>);

/// Enum storing value used to specific the version of a plugin to be downloaded by Lazy.
pub enum LazyVersion {
    /// Pin to a specific branch. Equivalent to `branch` in spec.
    Branch(&'static str),
    /// Pin to a commit. Equivalent to `commit` in spec
    Commit(&'static str),
    /// Pin to a tag. Equivalent to `tag` in spec.
    Tag(&'static str),
    /// Pin to a release or Semver. Equivalent to `version` in spec.
    Semver(&'static str),
}

/// Lazy loading configuration for plugin.
#[derive(Default)]
pub struct LazyLoad {
    lazy: bool,
    events: Option<&'static [&'static str]>,
    cmd: Option<&'static [&'static str]>,
    ft: Option<&'static [&'static str]>,
    keys: Vec<LazyKey>,
}

/// Lazy keybind for lazyloading.
#[derive(Default)]
pub struct LazyKey {
    key: &'static str,
    action: Option<Action>,
    desc: Option<&'static str>,
    modes: &'static [Mode],
}

/// A plugin to be loaded and download for lazy.
#[derive(Default)]
pub struct LazyPlugin {
    url: &'static str,
    dependencies: Option<&'static [&'static str]>,
    opts: Option<mlua::Table>,
    opts_extend: Option<&'static [&'static str]>,
    callback: Option<Box<dyn Fn(mlua::Table) -> nvim_oxi::Result<()>>>,
    main: Option<&'static str>,
    build: Option<&'static str>,
    version: Option<LazyVersion>,
    lazy_load: Option<LazyLoad>,
}

impl Lazy {
    /// Create a new Lazy instance to start managing plugin.
    #[must_use]
    pub fn new() -> Self {
        Self(vec![])
    }

    /// Add a plugin for Lazy to managing and download.
    pub fn add_plugin(&mut self, plugin: impl Into<LazyPlugin>) {
        self.0.push(plugin.into());
    }

    /// Add a collection of plugins for Lazy to managing and download.
    pub fn add_plugins(&mut self, plugins: Vec<impl Into<LazyPlugin>>) {
        for plugin in plugins {
            self.add_plugin(plugin);
        }
    }

    /// Bootstrap Lazy into neovim, download if not already on disk.
    fn bootstrap() -> nvim_oxi::Result<()> {
        // This code is simply a rewrite from the normal bootstrap script. It is only missing the
        // error report when lazy could not be install.
        // Refer to https://lazy.folke.io/installation for more info

        let lazypath =
            std::path::Path::new(&vim_fn::<String>("stdpath", "data")?).join("lazy/lazy.nvim");

        let lazypath_str = lazypath
            .clone()
            .into_os_string()
            .into_string()
            .unwrap()
            .replace('/', "\\");

        if !lazypath.exists() {
            std::process::Command::new("git")
                .args([
                    "clone",
                    "--filter=blob:none",
                    "--branch=stable",
                    "https://github.com/folke/lazy.nvim.git",
                    &lazypath_str,
                ])
                .spawn()
                .and_then(|mut c| c.wait())
                .expect("Cannot install lazy.nvim");
        }

        let old_rtp = nvim_oxi::api::get_option_value::<String>(
            "runtimepath",
            &nvim_oxi::api::opts::OptionOpts::default(),
        )?;

        nvim_oxi::api::set_option_value(
            "runtimepath",
            format!("{old_rtp},{lazypath_str}"),
            &nvim_oxi::api::opts::OptionOpts::default(),
        )?;

        Ok(())
    }

    /// Bootstrap and call set up for lazy with all specified plugin.
    pub fn setup(self) -> nvim_oxi::Result<()> {
        Self::bootstrap()?;

        let tbl = table! {
            change_detection = table! {
                enable = false,
                notify = false
            },
            rocks = table! {
                enabled = false
            }
        };

        tbl.set("spec", self.0)?;

        crate::require_setup("lazy", tbl)?;

        Ok(())
    }
}

impl Default for Lazy {
    fn default() -> Self {
        Self::new()
    }
}

impl LazyPlugin {
    /// Create a new plugin builder.
    #[must_use]
    pub fn new(url: &'static str) -> Self {
        Self {
            url,
            ..Self::default()
        }
    }

    /// Set the option for this plugin. Equivalent to `opts` in spec
    #[must_use]
    pub fn opts(mut self, opts: mlua::Table) -> Self {
        self.opts = Some(opts);
        self
    }

    /// Specified which plugin this plugin depend on to be load at the same time. Equivalent to
    /// `dependencies` in spec.
    #[must_use]
    pub fn depend(mut self, dependencies: &'static [&'static str]) -> Self {
        self.dependencies = Some(dependencies);
        self
    }

    /// Set a callback when this plugin is loaded to configure it. Equivalent to `config` in spec.
    #[must_use]
    pub fn callback(
        mut self,
        callback: impl Fn(mlua::Table) -> nvim_oxi::Result<()> + 'static,
    ) -> Self {
        self.callback = Some(Box::new(callback));
        self
    }

    /// Set a different name for the module to be automatically require when lazy call setup. Equivalent to `main`
    /// in spec.
    #[must_use]
    pub fn main(mut self, main: &'static str) -> Self {
        self.main = Some(main);
        self
    }

    /// Set a build command to be run after the plugin is installed or updated. Equivalent to
    /// `build` in spec.
    #[must_use]
    pub fn build(mut self, build: &'static str) -> Self {
        self.build = Some(build);
        self
    }

    /// Set a version to be pinned when lazy is installing or updating. Equivalent to all the
    /// version specifier: `branch`, `tag`, `commit`, `version`. Refer to [`LazyVersion`] for more
    /// info.
    #[must_use]
    pub fn version(mut self, version: LazyVersion) -> Self {
        self.version = Some(version);
        self
    }

    /// Specify how this plugin will be lazy load and the lazy loading configuration.
    #[must_use]
    pub fn lazy_load(mut self, lazy_load: LazyLoad) -> Self {
        self.lazy_load = Some(lazy_load);
        self
    }

    // TODO: Add documentation when this is finally documented by folke
    #[allow(missing_docs)]
    #[must_use]
    pub fn opts_extend(mut self, opt_extend: &'static [&'static str]) -> Self {
        self.opts_extend = Some(opt_extend);
        self
    }
}

impl LazyLoad {
    /// Create a new `LazyLoad` builder.
    #[must_use]
    pub fn new(lazy: bool) -> Self {
        Self {
            lazy,
            ..Self::default()
        }
    }

    /// Lazy load on events. Equivalent to `events` in spec.
    #[must_use]
    pub fn events(mut self, events: &'static [&'static str]) -> Self {
        self.events = Some(events);
        self
    }

    /// Lazy load on command execution. Equivalent to `cmd` in spec.
    #[must_use]
    pub fn cmd(mut self, cmd: &'static [&'static str]) -> Self {
        self.cmd = Some(cmd);
        self
    }

    /// Lazy load on file type. Equivalent to `ft` in spec.
    #[must_use]
    pub fn ft(mut self, ft: &'static [&'static str]) -> Self {
        self.ft = Some(ft);
        self
    }

    /// Lazy load on key map. Equivalent to `keys` in spec.
    #[must_use]
    pub fn add_key(mut self, key: impl Into<LazyKey>) -> Self {
        self.keys.push(key.into());
        self
    }

    /// Lazy load on key map. Equivalent to `keys` in spec.
    #[must_use]
    pub fn add_keys(mut self, key: Vec<LazyKey>) -> Self {
        self.keys.extend(key);
        self
    }
}

impl LazyKey {
    /// Create a new key map builder.
    #[must_use]
    pub fn new(key: &'static str) -> Self {
        Self {
            key,
            ..Self::default()
        }
    }

    /// The action to be executed by this keymap.
    #[must_use]
    pub fn action<I>(mut self, action: I) -> Self
    where
        I: Into<Action>,
    {
        self.action = Some(action.into());
        self
    }

    /// The mode this keymap is map in.
    pub fn modes(mut self, modes: &'static [Mode]) -> Self {
        self.modes = modes;
        self
    }
}

// implement easy config for plugin without much configuration
impl From<&'static str> for LazyPlugin {
    fn from(str: &'static str) -> Self {
        Self::new(str)
    }
}

impl IntoLua for LazyPlugin {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        let spec = lua.create_table()?;

        spec.push(self.url)?;

        spec.set(
            "opts",
            if let Some(opts) = self.opts {
                opts
            } else {
                table! {}
            },
        )?;

        if let Some(dependencies) = self.dependencies {
            spec.set("dependencies", dependencies)?;
        }
        if let Some(main) = self.main {
            spec.set("main", main)?;
        }
        if let Some(build) = self.build {
            spec.set("build", build)?;
        }

        if let Some(version) = self.version {
            match version {
                LazyVersion::Branch(b) => spec.set("branch", b)?,
                LazyVersion::Commit(c) => spec.set("commit", c)?,
                LazyVersion::Tag(t) => spec.set("tag", t)?,
                LazyVersion::Semver(v) => spec.set("version", v)?,
            }
        }

        if let Some(lazy_load) = self.lazy_load {
            spec.set("lazy", lazy_load.lazy)?;

            if let Some(events) = lazy_load.events {
                spec.set("event", events)?;
            }
            if let Some(cmd) = lazy_load.cmd {
                spec.set("cmd", cmd)?;
            }
            if let Some(ft) = lazy_load.ft {
                spec.set("ft", ft)?;
            }
            if !lazy_load.keys.is_empty() {
                let keys = table! {};
                for key in lazy_load.keys {
                    let k = table! {};

                    k.push(key.key)?;
                    k.push(key.action)?;
                    if let Some(desc) = key.desc {
                        k.set("desc", desc)?;
                    }

                    if !key.modes.is_empty() {
                        k.set(
                            "mode",
                            key.modes.iter().map(|m| m.as_char()).collect::<Vec<_>>(),
                        )?;
                    }
                    keys.push(k)?;
                }
                spec.set("keys", keys)?;
            }
        }

        if let Some(callback) = self.callback {
            spec.set(
                "config",
                lua.create_function(
                    move |_, (_, opt): (mlua::Table, mlua::Table)| match callback(opt) {
                        Ok(()) => Ok(()),
                        Err(err) => panic!("Error in config function of {}: {err}", self.url),
                    },
                )?,
            )?;
        }

        Ok(mlua::Value::Table(spec))
    }
}

impl From<&'static str> for LazyKey {
    fn from(str: &'static str) -> Self {
        Self::new(str)
    }
}
