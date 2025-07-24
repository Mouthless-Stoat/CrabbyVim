use crate::lua_table;
use mlua::{Function, Integer, Table};

pub fn configure() -> nvim_oxi::Result<()> {
    let config = nvim_oxi::mlua::lua()
        .globals()
        .get::<Table>("vim")?
        .get::<Table>("diagnostic")?
        .get::<Function>("config")?;

    let signs_text = lua_table! {};

    signs_text.set(DiagnosticSeverity::Error, "")?;
    signs_text.set(DiagnosticSeverity::Warn, "")?;
    signs_text.set(DiagnosticSeverity::Hint, "󰌵")?;
    signs_text.set(DiagnosticSeverity::Info, "")?;

    config.call::<()>(lua_table! {
        virtual_text = true,
        signs = lua_table!{
            text = signs_text
        }
    })?;

    Ok(())
}

enum DiagnosticSeverity {
    Error,
    Warn,
    Hint,
    Info,
}

impl mlua::IntoLua for DiagnosticSeverity {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        Ok(mlua::Value::Integer(
            lua.globals()
                .get::<Table>("vim")?
                .get::<Table>("diagnostic")?
                .get::<Table>("severity")?
                .get::<Integer>(match self {
                    Self::Error => "ERROR",
                    Self::Warn => "WARN",
                    Self::Hint => "HINT",
                    Self::Info => "INFO",
                })?,
        ))
    }
}
