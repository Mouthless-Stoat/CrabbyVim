use mlua::ObjectLike;

use crate::lazy::{LazyKey, LazyLoad, LazyPlugin};
use crate::{lua_table, require};

use super::Plugins;

pub fn plugins() -> Plugins {
    let opts = lua_table! {
        default_file_explorer = true,
        delete_to_trash = true,
        skip_confirm_for_simple_edits = true,

        keymaps = {
            ["q"] = { "actions.close" },
        },
        view_options = {
            show_hidden = true,
            highlight_filename = function(entry)
                local _, hl = require("nvim-web-devicons").get_icon(entry.name, vim.fn.fnamemodify(entry.name, ":e"))
                return hl
            end
        },
        float = {
            padding = 0,
            border = "single",
            preview_split = "right",
            override = function(conf)
                local uis = vim.api.nvim_list_uis()[1]
                local height = uis.height
                local width = uis.width

                conf.height = math.floor(height * 0.4)
                conf.width = width
                conf.row = height - conf.height

                conf.border = { "", "─", "", "", "", "", "", "" }
                conf.style = "minimal"
                return conf
            end,
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
    };

    Ok(vec![
        LazyPlugin::new("stevearc/oil.nvim")
            .opts(opts)
            .lazy_load(
                LazyLoad::new(false).add_key(LazyKey::new("<Leader>f").action(|| {
                    require("oil")?.call_function::<()>("toggle_float", ())?;
                    Ok(())
                })),
            ),
    ])
}
