use mlua::ObjectLike;

use crate::keymaps::Action;
use crate::theme::{HighlightOpt, set_hl};
use crate::{icons, lua_table, require, table};

pub fn config() -> nvim_oxi::Result<mlua::Table> {
    use icons::*;
    Ok(table! {
        prompt = format!(" {MAGNIFYING_GLASS} "),
        layouts = lua_table!{
            default = {
                layout = {
                    box = "horizontal",
                    backdrop = false,
                    height = 0.6,
                    {
                        box = "verticle",
                        border = "top",
                        title = "{title} {live} {flags}",
                        title_pos = "left",
                        { win = "input", height = 1},
                        { win = "list", border = "top" },
                    },
                    {
                        win = "preview",
                        border = {"─", "─", "─", "", "", "", "", "│"},
                        title = "{preview}",
                        title_pos = "left",
                        width = 0.65,
                    },
                },
            },
            select = {
                layout = {
                    box = "verticle",
                    border = "single",
                    backdrop = false,
                    width = 0.6,
                    title = "{title}",
                    title_pos = "left",
                    { win = "input", height = 1},
                    { win = "list", border = "top" },
                },
            },
        },
        previewers = lua_table!{
            diff = {
                builtin = false,
                cmd = { "delta" }
            },
        },
        icons = table! {
            kinds = table! {
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
        }
    })
}

pub fn highlights() -> nvim_oxi::Result<()> {
    use crate::theme::Color::*;
    #[rustfmt::skip]
    configure_picker_highligh(vec![
        ("Title", HighlightOpt::with_bg(Green).fg(Bg0)),

        ("PreviewTitle", HighlightOpt::with_bg(Blue).fg(Bg0)),
        ("ListTitle", HighlightOpt::with_bg(Yellow).fg(Bg0)),
        ("InputTitle", HighlightOpt::with_bg(Green).fg(Bg0)),
    ])?;

    Ok(())
}

pub fn picker(picker: &'static str) -> Action {
    (|| {
        Ok(require("snacks")?
            .get::<mlua::Table>("picker")?
            .call_function::<()>(picker, ())?)
    })
    .into()
}

fn configure_picker_highligh(hls: Vec<(&'static str, HighlightOpt)>) -> nvim_oxi::Result<()> {
    let hls = hls
        .into_iter()
        .map(|(name, opt)| (format!("SnacksPicker{name}"), opt))
        .collect::<Vec<(String, _)>>();

    for hl in hls {
        set_hl(hl.0.as_str(), hl.1)?;
    }
    Ok(())
}
