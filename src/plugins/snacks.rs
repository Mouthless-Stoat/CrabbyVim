use mlua::{ObjectLike, Table};

use crate::commands::create_command;
use crate::lazy::{LazyKey, LazyLoad, LazyPlugin};
use crate::plugins::Plugins;
use crate::theme::{HighlightOpt, configure_highlights};
use crate::{lua_table, require, table};

use super::delimiters::delimiter_highlights;

mod dashboard;
mod picker;
mod terminal;

pub(crate) fn plugins() -> Plugins {
    create_command(
        "Open the dashboard",
        "CrabbyDash",
        ":lua Snacks.dashboard()",
    )?;
    // TODO: replace this lua spam with rust function to be more "authentic"
    Ok(vec![
        LazyPlugin::new("folke/snacks.nvim")
            .depend(&["nvim-tree/nvim-web-devicons"])
            .opts(table! {
                indent = table!{
                    scope = table!{
                        hl = delimiter_highlights()
                    }
                },
                picker = picker::config()?,
                dashboard = dashboard::config()?,
                terminal = terminal::config()?,
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
                LazyLoad::new(true)
                    .events(&["VeryLazy"])
                    .add_keys(picker::key()?)
                    .add_keys(terminal::keys())
                    .add_key(LazyKey::new("<Leader>g").action(|| {
                        require("snacks")?
                            .get::<Table>("lazygit")?
                            .call_function::<()>("open", ())?;
                        Ok(())
                    })),
            ),
    ])
}

pub fn highlights() -> nvim_oxi::Result<()> {
    use crate::theme::Color::*;

    picker::highlights()?;
    dashboard::highlights()?;
    configure_highlights(vec![
        ("SnacksSpecial", HighlightOpt::with_fg(Blue)),
        ("SnacksIndent1", HighlightOpt::with_fg(Red)),
        ("SnacksIndent2", HighlightOpt::with_fg(Orange)),
        ("SnacksIndent3", HighlightOpt::with_fg(Yellow)),
        ("SnacksIndent4", HighlightOpt::with_fg(Green)),
        ("SnacksIndent5", HighlightOpt::with_fg(Cyan)),
        ("SnacksIndent6", HighlightOpt::with_fg(Blue)),
        ("SnacksIndent7", HighlightOpt::with_fg(Purple)),
    ])?;
    Ok(())
}
