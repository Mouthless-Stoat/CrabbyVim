use crate::lazy::{LazyPlugin, LazyVersion};
use crate::lua_table;
use crate::theme::{HighlightOpt, configure_highlights};

use super::Plugins;

pub(crate) fn plugins() -> Plugins {
    Ok(vec![
        LazyPlugin::new("j-hui/fidget.nvim")
            // check again later to not have to pin fidget
            .version(LazyVersion::Semver("v1.*"))
            .opts(lua_table! {
                progress = {
                    display = {
                        done_icon = "",
                        progress_icon = { "meter" },

                        done_style = "FidgetDone",
                        progress_style = "FidgetProgress",
                        group_style = "FidgetGroup",
                        icon_style = "FidgetIcon",
                    },
                },

                notification = {
                    filter = vim.log.levels.DEBUG,
                    window = {
                        normal_hl = "FidgetNormal",
                        winblend = 100,
                        x_padding = 1,
                        y_padding = 1,
                    },
                    override_vim_notify = true
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
        ("FidgetNormal", HighlightOpt::with_fg(Gray).bg(Bg1)),
    ])?;

    Ok(())
}
