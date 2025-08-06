use crate::{icons, lua_table, require, table};
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
        let vim_lsp = crate::vim()?.get::<Table>("lsp")?;
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

pub fn plugins() -> nvim_oxi::Result<Vec<crate::lazy::LazyPlugin>> {
    use crate::lazy::{LazyPlugin, LazyVersion};

    let kind_icons = table! {};

    let blink_opt = lua_table! {
        keymap = { preset = "super-tab" },
        completion = {
            ghost_text = { enabled = true },
            documentation = { auto_show = true },
            menu = {
                draw = {
                    padding = {0, 1},
                    columns = {{"kind_icon"}, {"label"}, {"kind"}},
                    components = {
                        label = {
                            text = function(ctx) return ctx.label end
                        },
                        kind = {
                            highlight = "Comment"
                        },
                        kind_icon = {
                            text = function(ctx) return " " .. ctx.kind_icon .. " " end
                        }
                    }
                }
            }
        },
        sources = { default = {"lsp", "path", "snippets", "buffer"} },
        fuzzy = { implementation = "rust" },
        signature = { enabled = true },
        snippets = { preset = "luasnip" }
    };

    blink_opt.set(
        "appearance",
        table! {
            nerd_font_variant = "mono",
            kind_icons = table! {
                Text = icons::TEXT,
                Method = icons::METHOD,
                Function = icons::FUNCTION,
                Constructor = icons::CONSTRUCTOR,

                Field = icons::FIELD,
                Variable = icons::VARIABLE,
                Property = icons::PROPERTY,

                Class = icons::CLASS,
                Interface = icons::INTERFACE,
                Struct = icons::STRUCT,
                Module = icons::MODULE,

                Unit = icons::UNIT,
                Value = icons::VALUE,
                Enum = icons::ENUM,
                EnumMember = icons::ENUM_MEMBER,

                Keyword = icons::KEYWORD,
                Constant = icons::CONSTANT,

                Snippet = icons::SNIPPET,
                Color = icons::COLOR,
                File = icons::FILE,
                Reference = icons::REFERENCE,
                Folder = icons::FOLDER,
                Event = icons::EVENT,
                Operator = icons::OPERATOR,
                TypeParameter = icons::TYPE_PARAMETER
            }
        },
    )?;

    // cheat using lua_table because there so many god damn table and funky function
    let blink = LazyPlugin::new("saghen/blink.cmp")
        .depend(&["neovim/nvim-lspconfig", "L3MON4D3/LuaSnip"])
        .version(LazyVersion::Semver("1.*"))
        .opts_extend(&["sources.default"])
        .opts(blink_opt);

    let luasnip = LazyPlugin::new("L3MON4D3/LuaSnip").version(LazyVersion::Semver("v2.*"));
    let mason = LazyPlugin::new("mason-org/mason.nvim").opts(lua_table! {
        ui = {
            icons = {
                package_installed = "",
                package_pending = "",
                package_uninstalled = ""
            }
        }
    });

    Ok(vec![blink, luasnip, mason])
}
