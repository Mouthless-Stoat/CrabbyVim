use crate::table;

use super::LazyPlugin;

pub fn plugin() -> nvim_oxi::Result<LazyPlugin> {
    Ok(LazyPlugin::new("stevearc/conform.nvim").opts(table! {
        formatters_by_ft = table! {
            lua = ["stylua"],
            python = ["black"],
            javascript = ["prettier"],
            typescript = ["prettier"],
            json = ["prettier"],
            markdown = ["prettier"],
            rust = ["rustfmt"],
            yaml = ["prettier"],
            toml = ["taplo"]
        },
        format_on_save = table! {
            timeout_ms = 500,
            lsp_format = "fallback"
        }
    }))
}
