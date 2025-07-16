use mlua::{Function, Table};
use nvim_oxi::api::opts::OptionOpts;
use std::env;

pub fn setup_lazy() -> nvim_oxi::Result<()> {
    let lazypath =
        std::path::Path::new(&env::var("XDG_CONFIG_HOME").unwrap()).join("lazy/lazy.nvim");
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
            .unwrap();
    }

    let old_rtp = nvim_oxi::api::get_option_value::<String>("runtimepath", &OptionOpts::default())?;

    nvim_oxi::api::set_option_value(
        "runtimepath",
        format!(
            "{old_rtp},{}",
            lazypath.to_str().unwrap().replace("/", "\\")
        ),
        &OptionOpts::default(),
    )?;

    let lua = nvim_oxi::mlua::lua();
    let lazy_setup: Function = lua
        .globals()
        .get::<Function>("require")?
        .call::<Table>("lazy")?
        .get::<Function>("setup")?;

    lazy_setup.call::<()>(lua.create_table())?;

    Ok(())
}
