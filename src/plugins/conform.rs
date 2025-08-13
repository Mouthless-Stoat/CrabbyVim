use crate::table;

use crate::lazy::{LazyLoad, LazyPlugin};
use crate::plugins::Plugins;

pub fn plugins() -> Plugins {
    Ok(vec![
        LazyPlugin::new("stevearc/conform.nvim")
            .opts(table! {
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
            })
            .lazy_load(LazyLoad::new(true).events(&["BufWritePre"])),
    ])
}
