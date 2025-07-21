use mlua::{Function, Table};

mod lazy;
mod lsp;
mod macros;
mod options;
mod theme;

#[derive(Clone, Copy)]
pub enum Mode {
    Normal,
    Insert,
    Command,
    Visual,
    Replace,
    Terminal,
}

impl Mode {
    fn current_mode() -> nvim_oxi::Result<Self> {
        Ok(match nvim_oxi::api::get_mode()?.mode.to_str().unwrap() {
            "n" | "niI" | "niR" | "niV" | "nt" | "ntT" => Self::Normal,
            "i" | "ic" | "ix" => Self::Insert,
            "v" | "vs" | "V" | "Vs" | "\u{16}" | "\u{16}s" | "s" | "S" | "\u{13}" => Self::Visual,
            "c" | "cv" | "ce" | "rm" | "r?" => Self::Command,
            "R" | "Rc" | "Rx" | "Rv" | "Rvc" | "Rvx" | "r" => Self::Replace,
            _ => Self::Terminal
        })
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Insert => "insert",
            Self::Command => "command",
            Self::Visual => "visual",
            Self::Replace => "replace",
            Self::Terminal => "terminal"
        }
    }
}

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
