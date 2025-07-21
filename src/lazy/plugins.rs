use crate::lua_table;
use mlua::Table;

pub struct Lazy(Vec<LazyPlugin>);

pub struct LazyPlugin {
    url: &'static str,
    dependencies: Option<&'static [&'static str]>,
    callback: Option<Box<dyn Fn() -> nvim_oxi::Result<()>>>,
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
                lazypath.to_str().unwrap().replace("/", "\\")
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

        let spec = lua_table! {};

        for plugin in self.0 {
            let tbl = lua_table! {};

            tbl.push(plugin.url)?;
            tbl.set("dependencies", plugin.dependencies)?;
            if let Some(callback) = plugin.callback {
                tbl.set(
                    "config",
                    lua.create_function(move |_, _: Table| Ok(callback().unwrap()))?,
                )?;
            }

            spec.push(tbl)?;
        }

        tbl.set("spec", spec)?;

        crate::require_setup("lazy", tbl)?;

        Ok(())
    }
}

impl LazyPlugin {
    fn new(url: &'static str) -> Self{
        LazyPlugin {
            url,
            dependencies: None,
            callback: None,
        }
    }

    fn depend(mut self, dependencies: &'static [&'static str]) -> Self {
        self.dependencies = Some(dependencies);
        self
    }

    fn callback(mut self, callback: impl Fn() -> nvim_oxi::Result<()> + 'static) -> Self{
        self.callback = Some(Box::new(callback));
        self
    }
}

impl From<&'static str> for LazyPlugin {
    fn from(str: &'static str) -> Self {
        Self::new(str)
    }
}
