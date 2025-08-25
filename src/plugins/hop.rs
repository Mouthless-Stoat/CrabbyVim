use mlua::ObjectLike;

use crate::{
    keymaps::Action,
    lazy::{LazyKey, LazyLoad, LazyPlugin},
    require, table,
    theme::{HighlightOpt, configure_highlights},
};

use super::Plugins;

pub fn plugins() -> Plugins {
    Ok(vec![
        LazyPlugin::new("smoka7/hop.nvim").lazy_load(
            LazyLoad::new(true)
                .add_key(LazyKey::new("<leader><leader>").action("<cmd>HopWord<cr>"))
                .add_key(LazyKey::new("f").action("<cmd>HopChar1<cr>"))
                .add_key(LazyKey::new("f").action(hop("hint_char1", table! {hint_offset = -1}))),
        ),
    ])
}

fn hop(func: &'static str, opts: mlua::Table) -> Action {
    (move || Ok(require("hop")?.call_function::<()>(func, opts.clone())?)).into()
}
