use mlua::{ObjectLike, Table};

use crate::lazy::{LazyKey, LazyLoad, LazyPlugin};
use crate::plugins::Plugins;
use crate::theme::{HighlightOpt, configure_highlights};
use crate::{lua_table, require, table};

use super::delimiters::delimiter_highlights;

mod picker;

pub fn plugins() -> Plugins {
    // TODO: replace this lua spam with rust function to be more "authentic"
    Ok(vec![
        LazyPlugin::new("folke/snacks.nvim")
            .depend(&[
                "nvim-tree/nvim-web-devicons",
                "aznhe21/actions-preview.nvim",
            ])
            .opts(table! {
                indent = table!{ 
                    scope = table!{
                        hl = delimiter_highlights()
                    }
                },
                picker = picker::config()?,
                lazygit = lua_table!{
                    config = {
                        os = { editPreset = "" }
                    },
                    win = {
                        backdrop = false
                    }
                }
            })
            .lazy_load(
                LazyLoad::new(false)
                    .add_keys(picker::key()?)
                    .add_key(LazyKey::new("<Leader>g").action(|| {
                        require("snacks")?
                            .get::<Table>("lazygit")?
                            .call_function::<()>("open", ())?;
                        Ok(())
                    }))
                    .add_key(LazyKey::new("<Leader>c").action(|| {
                        require("actions-preview")?.call_function::<()>("code_actions", ())?;
                        Ok(())
                    }))
            ),
        LazyPlugin::new("aznhe21/actions-preview.nvim").opts(lua_table! {
            highlight_command = {
                function() return require("actions-preview.highlight").delta("delta --paging=never") end
            },
            snacks = {
                focus = "list"
            }
        }),
    ])
}

pub fn highlights() -> nvim_oxi::Result<()> {
    use crate::theme::Color::*;

    picker::highlights()?;
    configure_highlights(vec![
        ("SnacksIndent1", HighlightOpt::with_fg(Red)),
        ("SnacksIndent2", HighlightOpt::with_fg(Orange)),
        ("SnacksIndent3", HighlightOpt::with_fg(Yellow)),
        ("SnacksIndent4", HighlightOpt::with_fg(Green)),
        ("SnacksIndent5", HighlightOpt::with_fg(Cyan)),
        ("SnacksIndent6", HighlightOpt::with_fg(Blue)),
        ("SnacksIndent7", HighlightOpt::with_fg(Purple)),
    ])      ?;
    Ok(())
}
