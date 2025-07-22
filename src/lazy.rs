use crate::lua_table;

mod plugins;

use plugins::*;

pub fn setup_lazy() -> nvim_oxi::Result<()> {
    let mut lazy = Lazy::new();

    lazy.add_plugin("neovim/nvim-lspconfig");

    lazy.add_plugin(
        LazyPlugin::new("nvim-treesitter/nvim-treesitter")
            .main("nvim-treesitter.configs")
            .version(LazyVersion::Branch("master"))
            .build(":TSUpdate")
            .opts(lua_table! {
                ensure_installed = [
                    "python",
                    "javascript",
                    "typescript",
                    "rust",
                    "gitcommit",
                    "gitignore",
                    "git_rebase",
                    "markdown"
                ],
                auto_install = true,
                highlight = lua_table! {
                    enable = true,
                    additional_vim_regex_highlighting = false
                },
                indent = lua_table! { enable = true }
            }),
    );

    lazy.setup()
}
