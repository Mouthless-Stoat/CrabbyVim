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

use crate::{lua_table, require, table};
use mlua::{Function, Table};
use nvim_oxi::mlua;

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

    lsp.configure()?;
    Ok(())
}

struct LspConfig {
    name: &'static str,
    settings: Table,
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
pub struct Lsp(Vec<LspConfig>);

impl Lsp {
    /// Create a new LSP structure to configure the LSP servers
    pub fn new() -> Self {
        Lsp(vec![])
    }

    /// Add a new LSP server config
    pub fn add_config(&mut self, config: LspConfig) {
        self.0.push(config);
    }

    /// Configure the LSP server with all config.
    pub fn configure(self) -> nvim_oxi::Result<()> {
        let vim_lsp = crate::vim()?.get::<Table>("lsp")?;
        let lsp_config = vim_lsp.get::<Table>("config")?;

        for config in self.0 {
            lsp_config.set(config.name, table! {
                settings = config.settings,
                capabilities = require("blink.cmp")?.get::<Function>("get_lsp_capabilities")?.call::<Table>(())?
            })?;
            vim_lsp.get::<Function>("enable")?.call::<()>(config.name)?;
        }

        Ok(())
    }
}
