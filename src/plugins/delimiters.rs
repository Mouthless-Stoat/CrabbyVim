use crate::lazy::LazyPlugin;
use crate::table;
use crate::theme::{HighlightOpt, set_hl};

use super::Plugins;

pub fn plugins() -> Plugins {
    Ok(vec![
        LazyPlugin::new("hiphish/rainbow-delimiters.nvim")
            .main("rainbow-delimiters.setup")
            .opts(table! {
                highlight = delimiter_highlights()
            }),
    ])
}

pub fn highlights() -> nvim_oxi::Result<()> {
    use crate::theme::Color::*;
    let colors = [Red, Orange, Yellow, Green, Cyan, Blue, Purple];

    for (name, color) in delimiter_highlights().into_iter().zip(colors) {
        set_hl(name, HighlightOpt::with_fg(color))?;
    }

    Ok(())
}

#[must_use]
pub fn delimiter_highlights() -> [&'static str; 7] {
    [
        "Delimit1", "Delimit2", "Delimit3", "Delimit4", "Delimit5", "Delimit6", "Delimit7",
    ]
}
