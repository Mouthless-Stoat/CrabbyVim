use mlua::ObjectLike;

use crate::autocmds::create_autocmd;
use crate::lazy::{LazyLoad, LazyPlugin};
use crate::plugins::Plugins;
use crate::{require, require_setup, table};

pub(crate) fn plugins() -> Plugins {
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
                    toml = ["taplo"],
                    typst = ["typstyle"]
                }
            })
            .callback(|opts| {
                require_setup("conform", opts)?;
                create_autocmd(&["BufWritePre"], &["*"], |args| {
                    require("conform")?.call_function::<bool>(
                        "format",
                        table! {
                            bufnr = args.buffer.handle(),
                            timeout_ms = 5000
                        },
                    )?;
                    Ok(())
                })?;

                Ok(())
            })
            .lazy_load(LazyLoad::new(true).events(&["BufWritePre"])),
    ])
}
