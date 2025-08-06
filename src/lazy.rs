use crate::table;

mod conform;
mod gitsigns;
mod plugins;
mod snacks;

pub use plugins::*;

pub fn setup_lazy() -> nvim_oxi::Result<()> {
    let mut lazy = Lazy::new();

    lazy.add_plugin("wakatime/vim-wakatime");

    lazy.add_plugin(snacks::plugin()?);

    // lsp and coding stuff
    lazy.add_plugins(crate::lsp::plugins()?);
    lazy.add_plugin(conform::plugin()?);
    lazy.add_plugin(
        LazyPlugin::new("nvim-treesitter/nvim-treesitter")
            .main("nvim-treesitter.configs")
            .version(LazyVersion::Branch("master"))
            .build(":TSUpdate")
            .opts(table! {
                ensure_installed = [
                    "python",
                    "javascript",
                    "typescript",
                    "rust",
                    "gitcommit",
                    "gitignore",
                    "git_rebase",
                    "git_config",
                    "markdown"
                ],
                auto_install = true,
                highlight = table! {
                    enable = true,
                    additional_vim_regex_highlighting = false
                },
                indent = table! { enable = true }
            }),
    );

    lazy.add_plugin(gitsigns::plugin()?);

    lazy.setup()
}
