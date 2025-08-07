use crate::lazy::{LazyPlugin, LazyVersion};
use crate::plugins::Plugins;
use crate::{icons, lua_table, table};

pub fn plugins() -> Plugins {
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

    Ok(vec![
        LazyPlugin::new("saghen/blink.cmp")
            .depend(&["neovim/nvim-lspconfig", "L3MON4D3/LuaSnip"])
            .version(LazyVersion::Semver("1.*"))
            .opts_extend(&["sources.default"])
            .opts(blink_opt),
    ])
}
