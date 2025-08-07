use crate::table;

use crate::lazy::LazyPlugin;

pub fn plugins() -> nvim_oxi::Result<Vec<LazyPlugin>> {
    Ok(vec![LazyPlugin::new("stevearc/conform.nvim").opts(
        table! {
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
                timeout_ms = 5000,
                lsp_format = "fallback"
            }
        },
    )])
}
