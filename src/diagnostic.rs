use crate::table;
use mlua::{Function, Integer, Table};

pub fn configure() -> nvim_oxi::Result<()> {
    let config = nvim_oxi::mlua::lua()
        .globals()
        .get::<Table>("vim")?
        .get::<Table>("diagnostic")?
        .get::<Function>("config")?;

    config.call::<()>(table! {
        virtual_text = true,
        signs = table!{
            text = table! {
                [DiagnosticSeverity::Error] = "",
                [DiagnosticSeverity::Warn] = "",
                [DiagnosticSeverity::Hint] = "󰌵",
                [DiagnosticSeverity::Info] = ""
            }
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
