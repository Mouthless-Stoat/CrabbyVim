use crate::lazy::{LazyLoad, LazyPlugin, LazyVersion};
use crate::lua_table;

use super::Plugins;

// The actually treesitter highlights are set in theme::syntax not here because it is cummbersome to
// move here with all the type and custom highlight group.

pub(crate) fn plugins() -> Plugins {
    Ok(vec![
        LazyPlugin::new("nvim-treesitter/nvim-treesitter")
            .main("nvim-treesitter.configs")
            .version(LazyVersion::Branch("master"))
            .build(":TSUpdate")
            .opts(lua_table! {
                ensure_installed = {
                    "python",
                    "javascript",
                    "typescript",
                    "rust",
                    "gitcommit",
                    "gitignore",
                    "git_rebase",
                    "git_config",
                    "markdown",
                    "diff"
                },
                auto_install = true,
                highlight = {
                    enable = true,
                    additional_vim_regex_highlighting = false
                },
                indent = { enable = true }
            })
            .lazy_load(
                LazyLoad::new(true)
                    .events(&["BufReadPost", "BufNewFile"])
                    .cmd(&["TSInstall", "TSUpdate"]),
            ),
    ])
}
