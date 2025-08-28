use mlua::ObjectLike;

use crate::{
    Mode,
    keymaps::Action,
    lazy::{LazyKey, LazyLoad, LazyPlugin},
    require, table,
};

use super::Plugins;

pub fn plugins() -> Plugins {
    Ok(vec![
        LazyPlugin::new("smoka7/hop.nvim").lazy_load(
            LazyLoad::new(true)
                .add_key(LazyKey::new("<leader><leader>").action("<cmd>HopWord<cr>"))
                .add_key(
                    LazyKey::new("s")
                        .action("<cmd>HopChar1<cr>")
                        .modes(Mode::nvo()),
                ),
        ),
    ])
}

fn hop(func: &'static str, opts: mlua::Table) -> Action {
    (move || Ok(require("hop")?.call_function::<()>(func, opts.clone())?)).into()
}
