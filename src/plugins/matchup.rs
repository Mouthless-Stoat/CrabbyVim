use crate::lazy::LazyPlugin;
use crate::lua_table;

use super::Plugins;

pub(crate) fn plugins() -> Plugins {
    Ok(vec![LazyPlugin::new("andymass/vim-matchup").opts(
        lua_table! {
            matchparen = {
                deferred = 1,
                hi_surround_always = 1,
                offscreen = {}
            },
            // conflict with mini.ai
            text_obj = {
                enabled = 0
            },
            transmute = {
                enabled = 1
            }
        },
    )])
}
