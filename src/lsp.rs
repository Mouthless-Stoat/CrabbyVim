use crate::lua_table;
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
            let tbl = lua.create_table()?;

            tbl.set("settings", config.settings)?;

            lsp_config.set(config.name, tbl)?;
            vim_lsp.get::<Function>("enable")?.call::<()>(config.name)?;
        }

        Ok(())
    }
}

pub fn setup_lsp() -> nvim_oxi::Result<()> {
    let lua = nvim_oxi::mlua::lua();
    let mut lsp = Lsp::new();

    lsp.add_config(LspConfig {
        name: "rust_analyzer",
        settings: lua_table! {
            imports = lua_table! { 
                granularity = lua_table! { group = "module" } ,
                prefix = "self"
            },
            cargo = lua_table! {
                buildScripts = lua_table! {
                    enable = true
                }
            },
            proMacro = lua_table! {
                enable = true
            },
            check = lua_table! {
                command = "clippy"
            }
        },
    });

    lsp.configure()?;
    Ok(())
}
