use mlua::{IntoLuaMulti, ObjectLike, Table};
use nvim_oxi::{Dictionary, api::types::LogLevel};

/// Helper to access the globals `vim` variable in lua.
pub fn vim() -> mlua::Result<mlua::Table> {
    nvim_oxi::mlua::lua().globals().get::<mlua::Table>("vim")
}

/// Helper to execute the `vim.notify` API for notification.
pub fn vim_notify(msg: &str, level: LogLevel) -> nvim_oxi::Result<()> {
    nvim_oxi::api::notify(msg, level, &Dictionary::new())?;
    Ok(())
}

/// Helper to execute `vim.fn` function or vimscript function.
pub fn vim_fn<T>(func: &'static str, args: impl IntoLuaMulti) -> nvim_oxi::Result<T>
where
    T: mlua::FromLua,
{
    Ok(vim()?.get::<Table>("fn")?.call_function::<T>(func, args)?)
}
