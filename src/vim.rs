use mlua::{IntoLuaMulti, ObjectLike, Table};
use nvim_oxi::Dictionary;
use nvim_oxi::api::types::LogLevel;

pub fn vim() -> mlua::Result<mlua::Table> {
    nvim_oxi::mlua::lua().globals().get::<mlua::Table>("vim")
}

pub fn vim_notify(msg: impl Into<String>, level: LogLevel) -> nvim_oxi::Result<()> {
    nvim_oxi::api::notify(&msg.into(), level, &Dictionary::new())?;
    Ok(())
}

pub fn vim_fn<T>(func: &'static str, args: impl IntoLuaMulti) -> nvim_oxi::Result<T>
where
    T: mlua::FromLua,
{
    Ok(vim()?.get::<Table>("fn")?.call_function::<T>(func, args)?)
}
