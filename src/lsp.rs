use crate::{lazy, require, table};
use mlua::{Function, Table};
use nvim_oxi::mlua;
use serde::Serialize;

#[derive(Serialize)]
struct LspConfig {
    name: &'static str,
    settings: Table,
}

pub struct Lsp {
    configs: Vec<LspConfig>,
}

impl Lsp {
    /// Create a new LSP structure to configure the LSP servers
    fn new() -> Self {
        Lsp { configs: vec![] }
    }

    /// Add a new LSP server config
    fn add_config(&mut self, config: LspConfig) {
        self.configs.push(config);
    }

    /// Configure the LSP server with all config.
    fn configure(self) -> nvim_oxi::Result<()> {
        let lua = mlua::lua();
        let vim = lua.globals().get::<Table>("vim")?;
        let vim_lsp = vim.get::<Table>("lsp")?;
        let lsp_config = vim_lsp.get::<Table>("config")?;

        for config in self.configs {
            lsp_config.set(config.name, table! {
                settings = config.settings,
                capabilities = require("blink.cmp")?.get::<Function>("get_lsp_capabilities")?.call::<Table>(())?
            })?;
            vim_lsp.get::<Function>("enable")?.call::<()>(config.name)?;
        }

        Ok(())
    }
}

pub fn setup_lsp() -> nvim_oxi::Result<()> {
    let mut lsp = Lsp::new();

    lsp.add_config(LspConfig {
        name: "rust_analyzer",
        settings: table! {
            imports = table! {
                granularity = table! { group = "module" } ,
                prefix = "self"
            },
            cargo = table! {
                buildScripts = table! {
                    enable = true
                }
            },
            proMacro = table! {
                enable = true
            },
            check = table! {
                command = "clippy"
            }
        },
    });

    lsp.configure()?;
    Ok(())
}

pub fn plugins() -> nvim_oxi::Result<crate::lazy::LazyPlugin> {
    use crate::lazy::{LazyPlugin, LazyVersion};
    Ok(LazyPlugin::new("saghen/blink.cmp")
        .depend(&["rafamadriz/friendly-snippets", "neovim/nvim-lspconfig"])
        .version(LazyVersion::Semver("1.*"))
        .opts(table! {
            keymap = table! { preset = "super-tab" },
            appearance = table! { nerd_font_variant = "mono" },
            completion = table! { documentation = table! { auto_show = false } },
            sources = table! { default = ["lsp", "path", "snippets", "buffer"] },
            fuzzy = table! { implementation = "rust" }
        }))
}
