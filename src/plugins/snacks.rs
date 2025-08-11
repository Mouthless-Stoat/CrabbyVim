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
            .depend(&[
                "nvim-tree/nvim-web-devicons",
                "aznhe21/actions-preview.nvim",
            ])
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
                    .add_key(LazyKey::new("<Leader>g").action(|| {
                        require("snacks")?
                            .get::<Table>("lazygit")?
                            .call_function::<()>("open", ())?;
                        Ok(())
                    }))
                    .add_key(LazyKey::new("<Leader>c").action(|| {
                        require("actions-preview")?.call_function::<()>("code_actions", ())?;
                        Ok(())
                    }))
            ),
        LazyPlugin::new("aznhe21/actions-preview.nvim").opts(lua_table! {
            highlight_command = {
                function() return require("actions-preview.highlight").delta("delta --paging=never") end
            },
            snacks = {
                focus = "list"
            }
        }),
    ])
}

pub fn highlights() -> nvim_oxi::Result<()> {
    picker::highlights()?;
    Ok(())
}
