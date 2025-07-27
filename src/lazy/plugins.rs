use mlua::IntoLua;

use crate::table;

pub struct Lazy(Vec<LazyPlugin>);

/// Enum storing used to specific the version of a plugin to be downloaded by Lazy
pub enum LazyVersion {
    // Pin to a specific branch. Equivalent to `branch` in spec.
    Branch(&'static str),
    // Pin to a commit. Equivalent to `commit` in spec
    Commit(&'static str),
    // Pin to a tag. Equivalent to `tag` in spec.
    Tag(&'static str),
    /// Pin to a release or Semver. Equivalent to `version` in spec.
    Semver(&'static str),
}

/// Lazy loading configuration for plugin
#[derive(Default)]
pub struct LazyLoad {
    lazy: bool,
    events: Option<&'static [&'static str]>,
    cmd: Option<&'static [&'static str]>,
    ft: Option<&'static [&'static str]>,
    keys: Option<mlua::Table>,
}

/// A plugin to be loaded and download for lazy
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
    /// Create a new Lazy instant to start managing plugin.
    pub fn new() -> Self {
        Self(vec![])
    }

    /// Add a plugins for Lazy to managing and download.
    pub fn add_plugin(&mut self, plugin: impl std::convert::Into<LazyPlugin>) {
        self.0.push(plugin.into());
    }

    /// Add a plugins for Lazy to managing and download.
    pub fn add_plugins(&mut self, plugins: Vec<impl std::convert::Into<LazyPlugin>>) {
        for plugin in plugins {
            self.0.push(plugin.into());
        }
    }

    /// Bootstrap Lazy into neovim, download if not already on disk.
    fn bootstrap() -> nvim_oxi::Result<()> {
        // Thsi code is simply a rewrite from the normal bootstrap script. It is only missing the
        // error report when lazy could not be install.
        // Refer to https://lazy.folke.io/installation for more info

        // TODO: replace enviroment variable access with using neovim api for `stdpath`
        let lazypath = std::path::Path::new(&std::env::var("XDG_DATA_HOME").unwrap())
            .join("nvim-data/lazy/lazy.nvim");

        if !lazypath.exists() {
            std::process::Command::new("git")
                .args([
                    "clone",
                    "--filter=blob:none",
                    "--branch=stable",
                    "https://github.com/folke/lazy.nvim.git",
                    lazypath.to_str().unwrap(),
                ])
                .spawn()
                .and_then(|mut c| c.wait())
                .expect("Cannot install lazy.nvim");
        }

        // TODO: clean this up. Appending to the option is way too messy
        let old_rtp = nvim_oxi::api::get_option_value::<String>(
            "runtimepath",
            &nvim_oxi::api::opts::OptionOpts::default(),
        )?;

        nvim_oxi::api::set_option_value(
            "runtimepath",
            format!(
                "{old_rtp},{}",
                lazypath.to_str().unwrap().replace('/', "\\")
            ),
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

impl LazyPlugin {
    /// Create a new builder.
    pub fn new(url: &'static str) -> Self {
        Self {
            url,
            ..Self::default()
        }
    }

    /// Set the option for this plugin. Equivalent to `opts` in spec
    pub fn opts(mut self, opts: mlua::Table) -> Self {
        self.opts = Some(opts);
        self
    }

    /// Specified which plugin this plugin depend on to be load at the same time. Equivalent to
    /// `dependencies` in spec.
    pub fn depend(mut self, dependencies: &'static [&'static str]) -> Self {
        self.dependencies = Some(dependencies);
        self
    }

    /// Set a callback when this plugin is loaded to configure it. Equivalent to `config` in spec.
    pub fn callback(
        mut self,
        callback: impl Fn(mlua::Table) -> nvim_oxi::Result<()> + 'static,
    ) -> Self {
        self.callback = Some(Box::new(callback));
        self
    }

    /// Set a different name for the module to be automatically require when lazy call setup. Equivalent to `main`
    /// in spec.
    pub fn main(mut self, main: &'static str) -> Self {
        self.main = Some(main);
        self
    }

    /// Set a build command to be run after the plugin is installed or updated. Equivalent to
    /// `build` in spec.
    pub fn build(mut self, build: &'static str) -> Self {
        self.build = Some(build);
        self
    }

    /// Set a version to be pinned when lazy is installing or updating. Equivalent to all the
    /// version specifier: `branch`, `tag`, `commit`, `version`. Refer to [`LazyVersion`] for more
    /// info.
    pub fn version(mut self, version: LazyVersion) -> Self {
        self.version = Some(version);
        self
    }

    /// Specify how this plugin will be lazy load and the lazy loading configuration.
    pub fn lazy_load(mut self, lazy_load: LazyLoad) -> Self {
        self.lazy_load = Some(lazy_load);
        self
    }

    pub fn opts_extend(mut self, opt_extend: &'static [&'static str]) -> Self {
        self.opts_extend = Some(opt_extend);
        self
    }
}

impl LazyLoad {
    /// Create a new LazyLoad builder.
    pub fn new(lazy: bool) -> Self {
        Self {
            lazy,
            ..Self::default()
        }
    }

    /// Lazy load on events. Equivalent to `events` in spec.
    pub fn events(mut self, events: &'static [&'static str]) -> Self {
        self.events = Some(events);
        self
    }

    /// Lazy load on command execution. Equivalent to `cmd` in spec.
    pub fn cmd(mut self, cmd: &'static [&'static str]) -> Self {
        self.cmd = Some(cmd);
        self
    }

    /// Lazy load on file type. Equivalent to `ft` in spec.
    pub fn ft(mut self, ft: &'static [&'static str]) -> Self {
        self.ft = Some(ft);
        self
    }

    /// Lazy load on key map. Equivalent to `keys` in spec.
    pub fn keys(mut self, keys: mlua::Table) -> Self {
        self.keys = Some(keys);
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

        if let Some(opts) = self.opts {
            spec.set("opts", opts)?;
        }

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
            if let Some(keys) = lazy_load.keys {
                spec.set("keys", keys)?;
            }
        }

        if let Some(callback) = self.callback {
            spec.set(
                "config",
                lua.create_function(move |_, opt: mlua::Table| match callback(opt) {
                    Ok(()) => Ok(()),
                    Err(err) => panic!("Error in config function of {}: {err}", self.url),
                })?,
            )?;
        }

        Ok(mlua::Value::Table(spec))
    }
}
