//! Configure and interact with the native lsp for neovim.
//!
//! configuration are done using the [`Lsp`] struct.
//!
//! # Examples
//!
//! ```rust
//! let mut lsp = Lsp::new();
//! lsp.add_config(LspConfig {
//!     name: "rust_analyzer",
//!     settings: lua_table! {
//!         ["rust-analyzer"] = {
//!             imports = {
//!                 granularity = { group = "module" } ,
//!                 prefix = "self"
//!             },
//!             cargo = {
//!                 buildScripts = {
//!                     enable = true
//!                 }
//!             },
//!             proMacro = {
//!                 enable = true
//!             },
//!             check = {
//!                 command = "clippy"
//!             }
//!         }
//!     },
//! });
//!
//! lsp.configure()?;
//! ```

use mlua::ObjectLike;
use mlua::Table;
use nvim_oxi::mlua;

use crate::autocmds::create_autocmd;
use crate::{Mode, lua_table, require, table, vim};

pub(crate) fn configure() -> nvim_oxi::Result<()> {
    let mut lsp = Lsp::new();

    lsp.add_config(LspConfig {
        name: "rust_analyzer",
        settings: lua_table! {
            ["rust-analyzer"] = {
                imports = {
                    granularity = { group = "module" } ,
                    prefix = "self"
                },
                cargo = {
                    buildScripts = {
                        enable = true
                    }
                },
                proMacro = {
                    enable = true
                },
                check = {
                    command = "clippy"
                }
            }
        },
    });

    create_autocmd(
        &["BufReadPre", "CmdlineEnter"],
        &["*"],
        move |_| -> nvim_oxi::Result<()> {
            lsp.configure()?;
            Ok(())
        },
    )?;

    create_autocmd(&["LspAttach"], &["*"], |mut args| {
        let mut set_key =
            |key: &'static str, name: &'static str, opt: mlua::Table| -> nvim_oxi::Result<()> {
                let mut opts = nvim_oxi::api::opts::SetKeymapOpts::builder();

                opts.silent(true);
                opts.callback(move |()| -> nvim_oxi::Result<()> {
                    vim()?
                        .get::<Table>("lsp")?
                        .get::<Table>("buf")?
                        .call_function::<()>(name, opt.clone())?;
                    Ok(())
                });

                args.buffer
                    .set_keymap(Mode::Normal.into(), key, "", &opts.build())?;
                Ok(())
            };

        set_key(
            "K",
            "hover",
            table! { close_events = ["CursorMoved", "BufLeave", "WinLeave"] },
        )?;

        set_key("<leader>c", "code_action", table! {})?;
        Ok(())
    })?;

    Ok(())
}

/// A struct to configure an LSP.
pub struct LspConfig {
    /// Name of the LSP.
    pub name: &'static str,
    /// Setting for the LSP.
    pub settings: Table,
}

/// Main struct for configuring and setting up LSP.
///
/// Creating a new instance with [`Lsp::new`] then add all the configuration using
/// [`Lsp::add_config`] then finally call [`Lsp::configure`] to configure the LSP
///
/// # Examples
///
/// ```rust
/// let mut lsp = Lsp::new();
/// lsp.add_config(LspConfig {
///     name: "rust_analyzer",
///     settings: lua_table! {
///         ["rust-analyzer"] = {
///             imports = {
///                 granularity = { group = "module" } ,
///                 prefix = "self"
///             },
///             cargo = {
///                 buildScripts = {
///                     enable = true
///                 }
///             },
///             proMacro = {
///                 enable = true
///             },
///             check = {
///                 command = "clippy"
///             }
///         }
///     },
/// });
///
/// lsp.configure()?;
/// ```
pub struct Lsp(Vec<LspConfig>, bool);

impl Lsp {
    /// Create a new LSP structure to configure the LSP servers
    #[must_use]
    pub fn new() -> Self {
        Lsp(vec![], false)
    }

    /// Add a new LSP server config
    pub fn add_config(&mut self, config: LspConfig) {
        self.0.push(config);
    }

    /// Configure the LSP server with all config.
    pub fn configure(&self) -> nvim_oxi::Result<()> {
        if self.1 {
            return Ok(());
        }
        let vim_lsp = crate::vim()?.get::<Table>("lsp")?;
        let lsp_config = vim_lsp.get::<Table>("config")?;

        for config in &self.0 {
            lsp_config.set(config.name, table! {
                settings = config.settings.clone(),
                capabilities = require("blink.cmp")?.get::<mlua::Function>("get_lsp_capabilities")?.call::<Table>(())?
            })?;
            vim_lsp
                .get::<mlua::Function>("enable")?
                .call::<()>(config.name)?;
        }

        Ok(())
    }
}

impl Default for Lsp {
    fn default() -> Self {
        Self::new()
    }
}
