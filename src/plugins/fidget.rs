use crate::lazy::{LazyPlugin, LazyVersion};
use crate::lua_table;
use crate::theme::{HighlightOpt, configure_highlights};

use super::Plugins;

pub fn plugins() -> Plugins {
    Ok(vec![
        LazyPlugin::new("j-hui/fidget.nvim")
            .version(LazyVersion::Semver("v1.*"))
            .opts(lua_table! {
                progress = {
                    display = {
                        done_ttl = 3,
                        done_icon = "",
                        progress_icon = { "meter" },

                        done_style = "FidgetDone",
                        progress_style = "FidgetProgress",
                        group_style = "FidgetGroup",
                        icon_style = "FidgetIcon",
                    },
                },

                notification = {
                    window = {
                        normal_hl = "FidgetNormal",
                        winblend = 50,
                        x_padding = 1,
                        y_padding = 1,
                    },
                },
            }),
    ])
}

pub fn highlights() -> nvim_oxi::Result<()> {
    use crate::theme::Color::*;

    configure_highlights(vec![
        ("FidgetDone", HighlightOpt::with_fg(Blue).italic()),
        ("FidgetProgress", HighlightOpt::with_fg(Green).italic()),
        ("FidgetGroup", HighlightOpt::with_fg(Yellow).italic()),
        ("FidgetIcon", HighlightOpt::with_fg(Green)),
        ("FidgetNormal", HighlightOpt::with_fg(Purple).bg(Bg1)),
    ])?;

    Ok(())
}
