use crate::plugins;

mod types;

pub use types::*;

pub fn setup_lazy() -> nvim_oxi::Result<()> {
    let mut lazy = Lazy::new();
    lazy.add_plugins(plugins()?);
    lazy.setup()
}
