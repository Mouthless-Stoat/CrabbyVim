use mlua::{IntoLuaMulti, ObjectLike, Table};

pub fn vim() -> mlua::Result<mlua::Table> {
    nvim_oxi::mlua::lua().globals().get::<mlua::Table>("vim")
}

pub fn vim_notify(msg: String) -> nvim_oxi::Result<()> {
    vim()?.call_function::<()>("notify", msg)?;
    Ok(())
}

pub fn vim_fn<T>(func: &'static str, args: impl IntoLuaMulti) -> nvim_oxi::Result<T>
where
    T: mlua::FromLua,
{
    Ok(vim()?.get::<Table>("fn")?.call_function::<T>(func, args)?)
}
