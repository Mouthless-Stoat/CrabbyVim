use mlua::ObjectLike;

use crate::keymaps::Action;
use crate::theme::HighlightOpt;
use crate::{lua_table, require};

use crate::lazy::{LazyKey, LazyLoad, LazyPlugin};

pub fn plugins() -> nvim_oxi::Result<Vec<LazyPlugin>> {
    // TODO: replace this lua spam with rust function to be more "authentic"
    Ok(vec![
        LazyPlugin::new("folke/snacks.nvim")
            .opts(lua_table! {
                picker = {},
            })
            .lazy_load(
                LazyLoad::new(true)
                    .add_key(LazyKey::new("<Leader>sf").action(snack_picker("files")))
                    .add_key(LazyKey::new("<Leader>st").action(snack_picker("grep")))
                    .add_key(
                        LazyKey::new("<Leader>ss").action(snack_picker("lsp_workspace_symbols")),
                    ),
            ),
    ])
}

fn snack_picker(picker: &'static str) -> Action {
    Action::Fn(Box::new(|| {
        Ok(require("snacks")?
            .get::<mlua::Table>("picker")?
            .call_function::<()>(picker, ())?)
    }))
}
