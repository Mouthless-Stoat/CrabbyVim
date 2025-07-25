use crate::{lua_table, require, table, theme::Color};
use mlua::{Function, Table};
use nvim_oxi::mlua;

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
        settings: lua_table! {
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
        },
    });

    lsp.configure()?;
    Ok(())
}

pub fn plugins() -> nvim_oxi::Result<crate::lazy::LazyPlugin> {
    use crate::lazy::{LazyPlugin, LazyVersion};
    // kind_icons  only include those are diff from normal
    Ok(LazyPlugin::new("saghen/blink.cmp")
        .depend(&["neovim/nvim-lspconfig"])
        .version(LazyVersion::Semver("1.*"))
        .opts_extend(&["sources.default"])
        .opts(lua_table! {
            keymap = { preset = "super-tab" },
            appearance = { 
                nerd_font_variant = "mono",
                kind_icons = {
                    Constructor = '󱌣',

                    Class = '',
                    Interface = '',
                    Struct = '',

                    Unit = '',
                    Enum = '',
                    EnumMember = '',

                    Snippet = '',
                    Operator = '',
                    TypeParameter = ''
                }
            },
            completion = {
                ghost_text = { enabled = true },
                documentation = { auto_show = true },
                menu = {
                    draw = {
                        columns = {{"kind_icon"}, {"label", "kind"}},
                        treesitter = {"lsp"},
                        components = {
                            label = {
                                text = function(ctx) return ctx.label end
                            }
                        }
                    }
                }
            },
            sources = { default = {"lsp", "path", "snippets", "buffer"} },
            fuzzy = { implementation = "rust" },
            signature = { enabled = true }
        }))
}
