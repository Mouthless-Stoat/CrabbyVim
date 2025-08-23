//! This is the main crate for `CrabbyVim`. [`crate::config`] is the main entry of the config that
//! configure everything from the options to keymaps to plugins and lsp. Each of the module
//! configure a different part of the config and can be enable/disable simply by commenting the
//! `configure()` call out. Each module also contain helper for configure them.

use mlua::{Function, Table};
use nvim_oxi::api::types::Mode as OxiMode;

pub mod autocmds;
pub mod commands;
pub mod diagnostic;
pub mod icons;
pub mod keymaps;
pub mod lazy;
pub mod lsp;
pub mod macros;
pub mod options;
pub mod status;
pub mod theme;

mod vim;
pub use vim::*;

pub mod plugins;

#[nvim_oxi::plugin]
/// The main entry point of the config. This function configure everything simple comment out each
/// module `configure()` call to disable it.
pub fn config() -> nvim_oxi::Result<()> {
    options::configure()?;
    keymaps::configure()?;

    let mut lazy = lazy::Lazy::new();
    lazy.add_plugins(plugins::plugins()?);
    lazy.setup()?;

    lsp::configure()?;
    theme::configure()?;
    status::configure()?;

    diagnostic::configure()?;
    commands::configure()?;
    autocmds::configure()?;

    Ok(())
}

#[derive(Clone, Copy)]
/// An enum to for which mode Neovim is in.
#[allow(missing_docs)]
pub enum Mode {
    Normal,
    Insert,
    Command,
    Visual,
    Replace,
    Terminal,
}

impl Mode {
    /// Fetch the current mode of the neovim instance
    pub fn current_mode() -> nvim_oxi::Result<Self> {
        Ok(match nvim_oxi::api::get_mode().mode.to_str().unwrap() {
            "n" | "niI" | "niR" | "niV" | "nt" | "ntT" => Self::Normal,
            "i" | "ic" | "ix" => Self::Insert,
            "v" | "vs" | "V" | "Vs" | "\u{16}" | "\u{16}s" | "s" | "S" | "\u{13}" => Self::Visual,
            "c" | "cv" | "ce" | "rm" | "r?" => Self::Command,
            "R" | "Rc" | "Rx" | "Rv" | "Rvc" | "Rvx" | "r" => Self::Replace,
            _ => Self::Terminal,
        })
    }

    #[must_use]
    /// Convert the enum into a string representing which mode it is.
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

/// Helper for require method.
pub fn require(module: &str) -> nvim_oxi::Result<Table> {
    Ok(nvim_oxi::mlua::lua()
        .globals()
        .get::<Function>("require")?
        .call::<Table>(module)?)
}

/// Helper for require and setup method.
pub fn require_setup(module: &str, opts: impl mlua::IntoLua) -> nvim_oxi::Result<()> {
    nvim_oxi::mlua::lua()
        .globals()
        .get::<Function>("require")?
        .call::<Table>(module)?
        .get::<Function>("setup")?
        .call::<()>(opts)?;

    Ok(())
}
