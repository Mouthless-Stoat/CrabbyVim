use mlua::{Function, Table};

mod lazy;
mod lsp;
mod macros;
mod options;
mod theme;

#[nvim_oxi::plugin]
fn config() -> nvim_oxi::Result<()> {
    lazy::setup_lazy()?;
    lsp::setup_lsp()?;
    options::configure_options()?;
    theme::configure_highlight()?;

    Ok(())
}

/// Helper for require and setup method
pub fn require_setup(module: &str, opts: impl mlua::IntoLua) -> nvim_oxi::Result<()> {
    nvim_oxi::mlua::lua()
        .globals()
        .get::<Function>("require")?
        .call::<Table>(module)?
        .get::<Function>("setup")?
        .call::<()>(opts)?;

    Ok(())
}
