use crate::lua_table;

use super::{LazyLoad, LazyPlugin};

pub fn plugin() -> nvim_oxi::Result<LazyPlugin> {
    // TODO: replace this lua spam with rust function to be more "authentic"
    Ok(LazyPlugin::new("folke/snacks.nvim")
        .opts(lua_table! {
            picker = {},
        })
        .lazy_load(LazyLoad::new(true).keys(lua_table! {
            {"<Leader>sf", function() require("snacks").picker.files() end, desc = "[s]earch [f]ile"},
            {"<Leader>st", function() require("snacks").picker.grep() end, desc = "[s]earch [t]ext"},
            {"<Leader>ss", function() require("snacks").picker.lsp_workspace_symbols() end, desc = "[s]earch [s]ymbols"}
        })))
}
