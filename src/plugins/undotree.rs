use nvim_oxi::api::{Buffer, set_var};

use crate::Mode;
use crate::lazy::{LazyKey, LazyLoad, LazyPlugin};

use super::Plugins;

pub fn plugins() -> Plugins {
    Ok(vec![
        LazyPlugin::new("mbbill/undotree")
            .callback(|_opts| {
                set_var("undotree_WindowLayout", 2)?;
                set_var("undotree_SetFocusWhenToggle", 1)?;
                set_var("undotree_DiffCommand", "git diff")?;
                set_var("undotree_SplitWidth", 50)?;

                Ok(())
            })
            .lazy_load(
                LazyLoad::new(true).add_key(LazyKey::new("<leader>u").action(|| {
                    nvim_oxi::api::command("UndotreeToggle")?;
                    Ok(())
                })),
            ),
    ])
}
