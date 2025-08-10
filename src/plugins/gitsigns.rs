use crate::theme::{HighlightOpt, configure_highlights};
use crate::{icons, table};

use crate::lazy::LazyPlugin;
use crate::plugins::Plugins;

pub fn plugins() -> Plugins {
    let signs_table = table! {
        add = table!{text = icons::ADDED},
        change = table!{text = icons::CHANGED},
        delete = table!{text = icons::DELETED},
        topdelete = table!{text = icons::TOP_DELETED},
        changedelete = table!{text = icons::CHANGE_DELETED},
        untracked = table!{text = icons::UNTRACKED}
    };

    Ok(vec![LazyPlugin::new("lewis6991/gitsigns.nvim").opts(
        table! {
            signs = &signs_table,
            signs_staged = signs_table,
            numhl = true,
            attach_to_untracked = true
        },
    )])
}

pub fn highlights() -> nvim_oxi::Result<()> {
    use crate::theme::Color::*;
    configure_highlights(vec![
        ("GitSignsUntracked", HighlightOpt::with_fg(Purple)),
        ("GitSignsUntrackedNr", HighlightOpt::link("LineNr")),
    ])?;
    Ok(())
}
