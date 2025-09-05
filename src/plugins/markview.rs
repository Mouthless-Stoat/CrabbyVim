use crate::lazy::{LazyLoad, LazyPlugin};
use crate::theme::{Color, HighlightOpt, set_hl};
use crate::{icons, lua_table, table};

use super::Plugins;

pub fn plugins() -> Plugins {
    Ok(vec![
        LazyPlugin::new("OXY2DEV/markview.nvim")
            .opts(table! {
                preview = lua_table!{
                    modes = {"i", "n", "c", "no"},
                    hybrid_modes = { "n", "i" },
                    raw_previews = {
                        markdown = { "tables" },

                        markdown_inline = {},
                        html = {},
                        latex = {},
                        typst = {},
                        yaml = {},
                    }
                },
                markdown = renderer_config()?,
                typst = table!{ enable = false }
            })
            .lazy_load(LazyLoad::new(true)),
    ])
}

fn renderer_config() -> nvim_oxi::Result<mlua::Table> {
    let headings = table! {};
    for (n, icon) in (1..=6).zip([
        icons::ONE,
        icons::TWO,
        icons::THREE,
        icons::FOUR,
        icons::FIVE,
    ]) {
        headings.set(
            format!("heading_{n}"),
            table! {
                icon = icon
            },
        )?;
    }

    Ok(table! {
        headings = headings
    })
}

pub fn highlights() -> nvim_oxi::Result<()> {
    use Color::*;

    set_markview_palette(0, Gray)?;
    set_markview_palette(1, Red)?;
    set_markview_palette(2, Yellow)?;
    set_markview_palette(3, Purple)?;
    set_markview_palette(4, Green)?;
    set_markview_palette(5, Blue)?;
    set_markview_palette(6, Cyan)?;

    Ok(())
}

fn set_markview_palette(num: usize, color: Color) -> nvim_oxi::Result<()> {
    set_hl(
        format!("markviewPalette{num}Fg"),
        HighlightOpt::with_fg(color),
    )?;
    Ok(())
}
