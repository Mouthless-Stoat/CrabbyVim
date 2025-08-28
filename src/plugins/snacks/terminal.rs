use mlua::{ObjectLike, Table};

use crate::lazy::LazyKey;
use crate::{lua_table, require};

pub fn config() -> nvim_oxi::Result<mlua::Table> {
    Ok(lua_table! {
        win = {
            height = function(win)
                if win.opts.position == "bottom" then
                    return 0.6
                end
                return 0.9
            end,
            row = function(win)
                if win.opts.position == "bottom" then
                    return 0.6
                end
                return 1
            end,
            wo = { winbar = " " },
            keys = { ["<C-`>"] = { "hide", mode = "t" } },
        }
    })
}

pub fn keys() -> std::vec::Vec<LazyKey> {
    vec![LazyKey::new("<C-`>").action(|| {
        require("snacks")?
            .get::<Table>("terminal")?
            .call_function::<()>("toggle", ())?;
        Ok(())
    })]
}
