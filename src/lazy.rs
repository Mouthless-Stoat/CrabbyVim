mod plugins;

use plugins::*;

pub fn setup_lazy() -> nvim_oxi::Result<()> {
    let mut lazy = Lazy::new();

    lazy.add_plugin("neovim/nvim-lspconfig");

    lazy.setup()
}
