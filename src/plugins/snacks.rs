use crate::plugins::Plugins;
use crate::table;

use crate::lazy::{LazyKey, LazyLoad, LazyPlugin};

mod picker;
use picker::picker;

pub fn plugins() -> Plugins {
    // TODO: replace this lua spam with rust function to be more "authentic"
    Ok(vec![
        LazyPlugin::new("folke/snacks.nvim")
            .opts(table! {
                picker = picker::config()?,
                terminal = table!{}
            })
            .lazy_load(
                LazyLoad::new(false)
                    .add_key(LazyKey::new("<Leader>sf").action(picker("files")))
                    .add_key(LazyKey::new("<Leader>st").action(picker("grep")))
                    .add_key(LazyKey::new("<Leader>ss").action(picker("lsp_workspace_symbols")))
                    .add_key(LazyKey::new("<Leader>su").action(picker("undo"))),
            ),
    ])
}
