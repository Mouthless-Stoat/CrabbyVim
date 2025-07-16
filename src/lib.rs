use nvim_oxi::api::opts::OptionOpts;
use nvim_oxi::mlua::{
    self,
    prelude::{LuaFunction, LuaTable},
};
use std::env;

#[nvim_oxi::plugin]
fn config() -> nvim_oxi::Result<()> {
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

    let lua = mlua::lua();
    let lazy_setup: LuaFunction = lua
        .globals()
        .get::<LuaFunction>("require")?
        .call::<LuaTable>("lazy")?
        .get::<LuaFunction>("setup")?;

    lazy_setup.call::<()>(lua.create_table())?;

    Ok(())
}
