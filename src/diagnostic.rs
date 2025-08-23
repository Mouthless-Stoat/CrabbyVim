//! Configure diagnostic for Neovim
use crate::{icons, table};
use mlua::{Function, Integer, Table};

pub(crate) fn configure() -> nvim_oxi::Result<()> {
    let config = crate::vim()?
        .get::<Table>("diagnostic")?
        .get::<Function>("config")?;

    config.call::<()>(table! {
        virtual_text = table! {
            prefix = " "
        },
        signs = table!{
            text = table! {
                [DiagnosticSeverity::Error] = icons::ERROR,
                [DiagnosticSeverity::Warn] = icons::WARN,
                [DiagnosticSeverity::Hint] = icons::HINT,
                [DiagnosticSeverity::Info] = icons::INFO
            },
            num_hl = table! {
                [DiagnosticSeverity::Error] = "DiagnosticError",
                [DiagnosticSeverity::Warn] = "DiagnosticWarn",
                [DiagnosticSeverity::Hint] = "DiagnosticHint",
                [DiagnosticSeverity::Info] = "DiagnosticInfo"
            }
        }
    })?;

    Ok(())
}

/// Enum representing the level of severity for each Neovim diagnostic
#[allow(missing_docs)]
pub enum DiagnosticSeverity {
    Error,
    Warn,
    Hint,
    Info,
}

impl mlua::IntoLua for DiagnosticSeverity {
    fn into_lua(self, _lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        Ok(mlua::Value::Integer(
            crate::vim()?
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
