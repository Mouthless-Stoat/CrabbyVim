use crate::lua_table;
use mlua::Table;

pub struct Lazy(Vec<LazyPlugin>);

enum LazyVersion {
    Branch(&'static str),
    Commit(&'static str),
    Tag(&'static str),
    Semver(&'static str),
}

struct LazyLoad {
    lazy: bool,
    event: Option<&'static [&'static str]>,
    cmd: Option<&'static [&'static str]>,
    ft: Option<&'static [&'static str]>,
    keys: Option<&'static [&'static str]>,
}

#[derive(Default)]
pub struct LazyPlugin {
    url: &'static str,
    dependencies: Option<&'static [&'static str]>,
    callback: Option<Box<dyn Fn() -> nvim_oxi::Result<()>>>,
    main: Option<&'static str>,
    build: Option<&'static str>,
    version: Option<LazyVersion>,
    lazy_load: Option<LazyLoad>,
}

impl Lazy {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn add_plugin(&mut self, plugin: impl std::convert::Into<LazyPlugin>) {
        self.0.push(plugin.into());
    }

    fn bootstrap() -> nvim_oxi::Result<()> {
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

    pub fn setup(self) -> nvim_oxi::Result<()> {
        Self::bootstrap()?;

        let lua = nvim_oxi::mlua::lua();

        let tbl = lua_table! {
            change_detection = lua_table! {
                enable = false,
                notify = false
            }
        };

        let plugins_spec = lua_table! {};

        for plugin in self.0 {
            let spec = lua_table! {};

            spec.push(plugin.url)?;

            if let Some(dependencies) = plugin.dependencies {
                spec.set("dependencies", dependencies)?
            };
            if let Some(main) = plugin.main {
                spec.set("main", main)?
            };
            if let Some(build) = plugin.build {
                spec.set("build", build)?
            };

            if let Some(version) = plugin.version {
                match version {
                    LazyVersion::Branch(b) => spec.set("branch", b)?,
                    LazyVersion::Commit(c) => spec.set("commit", c)?,
                    LazyVersion::Tag(t) => spec.set("tag", t)?,
                    LazyVersion::Semver(v) => spec.set("version", v)?,
                }
            };

            if let Some(lazy_load) = plugin.lazy_load {
                spec.set("lazy", lazy_load.lazy)?;

                if let Some(event) = lazy_load.event {
                    spec.set("event", event)?;
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

            if let Some(callback) = plugin.callback {
                spec.set(
                    "config",
                    lua.create_function(move |_, _: Table| {
                        callback().unwrap();
                        Ok(())
                    })?,
                )?;
            }

            plugins_spec.push(spec)?;
        }

        tbl.set("spec", plugins_spec)?;

        crate::require_setup("lazy", tbl)?;

        Ok(())
    }
}

impl LazyPlugin {
    fn new(url: &'static str) -> Self {
        LazyPlugin {
            url,
            ..LazyPlugin::default()
        }
    }

    fn depend(mut self, dependencies: &'static [&'static str]) -> Self {
        self.dependencies = Some(dependencies);
        self
    }

    fn callback(mut self, callback: impl Fn() -> nvim_oxi::Result<()> + 'static) -> Self {
        self.callback = Some(Box::new(callback));
        self
    }
}

impl From<&'static str> for LazyPlugin {
    fn from(str: &'static str) -> Self {
        Self::new(str)
    }
}
