use crate::keymaps::Action;
use crate::plugins::Plugins;
use crate::{lua_table, require, table};

use crate::lazy::{LazyKey, LazyLoad, LazyPlugin};

mod picker;
use mlua::{ObjectLike, Table};
use picker::picker;

pub fn plugins() -> Plugins {
    // TODO: replace this lua spam with rust function to be more "authentic"
    Ok(vec![
        LazyPlugin::new("folke/snacks.nvim")
            .depend(&["nvim-tree/nvim-web-devicons"])
            .opts(table! {
                picker = picker::config()?,
                lazygit = lua_table!{
                    config = {
                        os = { editPreset = "" }
                    },
                }
            })
            .lazy_load(
                LazyLoad::new(false)
                    .add_key(LazyKey::new("<Leader>sf").action(picker("files")))
                    .add_key(LazyKey::new("<Leader>st").action(picker("grep")))
                    .add_key(LazyKey::new("<Leader>ss").action(picker("lsp_workspace_symbols")))
                    .add_key(LazyKey::new("<Leader>su").action(picker("undo")))
                    .add_key(LazyKey::new("<Leader>g").action(Action::Fn(Box::new(|| {
                        require("snacks")?
                            .get::<Table>("lazygit")?
                            .call_function::<()>("open", ())?;
                        Ok(())
                    })))),
            ),
    ])
}

pub fn highlights() -> nvim_oxi::Result<()> {
    picker::highlights()?;
    Ok(())
}
