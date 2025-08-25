use crate::lazy::{LazyKey, LazyLoad, LazyPlugin};
use crate::lua_table;

use super::Plugins;

pub(crate) fn plugins() -> Plugins {
    let opts = lua_table! {
        default_file_explorer = true,
        delete_to_trash = true,
        skip_confirm_for_simple_edits = true,

        keymaps = {
            ["q"] = { "actions.close" },
            ["<C-s>"] = false,
            ["<C-h>"] = false
        },
        view_options = {
            show_hidden = true,
            highlight_filename = function(entry)
                local _, hl = require("nvim-web-devicons").get_icon(entry.name, vim.fn.fnamemodify(entry.name, ":e"))
                return hl
            end
        },
    };

    Ok(vec![
        LazyPlugin::new("stevearc/oil.nvim")
            .opts(opts)
            .lazy_load(LazyLoad::new(false).add_key(LazyKey::new("-").action("<cmd>Oil<cr>"))),
    ])
}
