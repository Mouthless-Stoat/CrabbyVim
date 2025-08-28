use mlua::{ObjectLike, Table};

use crate::lazy::LazyKey;
use crate::{lua_table, require};

pub fn config() -> nvim_oxi::Result<mlua::Table> {
    Ok(lua_table! {
        win = {
            layout = {
                box = "horizontal",
                backdrop = false,
                height = 0.6,
                row = -1,
            },
            keys = { ["<C-`>"] = { "hide", mode = "t" } },
            wo = {
                winbar = ""
            }
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
