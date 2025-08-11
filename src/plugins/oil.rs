use mlua::ObjectLike;

use crate::lazy::{LazyKey, LazyLoad, LazyPlugin};
use crate::{lua_table, require};

use super::Plugins;

pub fn plugins() -> Plugins {
    Ok(vec![
        LazyPlugin::new("stevearc/oil.nvim")
            .opts(lua_table! {
                default_file_explorer = true,
                delete_to_trash = true,
                skip_confirm_for_simple_edits = true,

                keymaps = {
                    ["<space>"] = { "actions.select" },
                    ["q"] = { "actions.close", mode = "n" },
                },
                view_options = {
                    show_hidden = true,
                    highlight_filename = function(entry)
                        local _, hl = require("nvim-web-devicons").get_icon(entry.name, vim.fn.fnamemodify(entry.name, ":e"))
                        return hl
                    end
                },
                preview_win = {
                    border = "single"
                },
                confirmation = {
                    border = "single"
                },
                progress = {
                    border = "single"
                }
            })
            .lazy_load(
                LazyLoad::new(true)
                    .add_key(LazyKey::new("<Leader>f").action(|| {
                        require("oil")?.call_function::<()>("open", ())?;
                        Ok(())
                    })),
            ),
    ])
}
