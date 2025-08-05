use mlua::ObjectLike;

use crate::keymaps::Action;
use crate::{lua_table, require};

use super::{LazyKey, LazyLoad, LazyPlugin};

pub fn plugin() -> nvim_oxi::Result<LazyPlugin> {
    // TODO: replace this lua spam with rust function to be more "authentic"
    Ok(LazyPlugin::new("folke/snacks.nvim")
        .opts(lua_table! {
            picker = {},
        })
        .lazy_load(
            LazyLoad::new(true)
                .add_key(LazyKey::new("<Leader>sf").action(snack_picker("files")))
                .add_key(LazyKey::new("<Leader>st").action(snack_picker("grep")))
                .add_key(LazyKey::new("<Leader>ss").action(snack_picker("lsp_workspace_symbols"))),
        ))
}

fn snack_picker(picker: &'static str) -> Action {
    Action::Fn(Box::new(|| {
        Ok(require("snacks")?
            .get::<mlua::Table>("picker")?
            .call_function::<()>(picker, ())?)
    }))
}
