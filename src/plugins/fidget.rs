use mlua::Function;

use crate::lazy::{LazyPlugin, LazyVersion};
use crate::options::set_option;
use crate::theme::{HighlightOpt, configure_highlights};
use crate::{lua_table, require, require_setup, vim};

use super::Plugins;

pub fn plugins() -> Plugins {
    Ok(vec![
        LazyPlugin::new("j-hui/fidget.nvim")
            // check again later to not have to pin fidget
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
            })
            .callback(|opts| {
                require_setup("fidget", opts)?;
                vim()?.set(
                    "notify",
                    require("fidget.notification")?.get::<Function>("notify")?,
                )?;
                Ok(())
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
