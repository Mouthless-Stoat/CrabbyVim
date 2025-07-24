use mlua::{Function, Table};
use nvim_oxi::api::types::Mode as OxiMode;

mod autocmds;
mod commands;
mod diagnostic;
mod keymaps;
mod lazy;
mod lsp;
mod macros;
mod options;
mod theme;

#[nvim_oxi::plugin]
fn config() -> nvim_oxi::Result<()> {
    lazy::setup_lazy()?;
    lsp::setup_lsp()?;

    diagnostic::configure()?;

    options::configure()?;
    keymaps::configure()?;

    theme::configure()?;
    commands::configure()?;

    autocmds::configure()?;

    Ok(())
}

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
    pub fn current_mode() -> nvim_oxi::Result<Self> {
        Ok(match nvim_oxi::api::get_mode()?.mode.to_str().unwrap() {
            "n" | "niI" | "niR" | "niV" | "nt" | "ntT" => Self::Normal,
            "i" | "ic" | "ix" => Self::Insert,
            "v" | "vs" | "V" | "Vs" | "\u{16}" | "\u{16}s" | "s" | "S" | "\u{13}" => Self::Visual,
            "c" | "cv" | "ce" | "rm" | "r?" => Self::Command,
            "R" | "Rc" | "Rx" | "Rv" | "Rvc" | "Rvx" | "r" => Self::Replace,
            _ => Self::Terminal,
        })
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Insert => "insert",
            Self::Command => "command",
            Self::Visual => "visual",
            Self::Replace => "replace",
            Self::Terminal => "terminal",
        }
    }
}

impl From<Mode> for OxiMode {
    fn from(value: Mode) -> Self {
        match value {
            Mode::Normal => OxiMode::Normal,
            Mode::Insert => OxiMode::Insert,
            Mode::Command => OxiMode::CmdLine,
            Mode::Visual => OxiMode::Visual,
            Mode::Terminal => OxiMode::Terminal,
            Mode::Replace => panic!("You can't convert from replace mode"),
        }
    }
}

/// Helper for require and setup method
pub fn require(module: &str) -> nvim_oxi::Result<Table> {
    nvim_oxi::mlua::lua()
        .globals()
        .get::<Function>("require")?
        .call::<Table>(module)?;
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
