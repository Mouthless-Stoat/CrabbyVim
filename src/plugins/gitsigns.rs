use crate::{icons, table};

use crate::lazy::LazyPlugin;

pub fn plugins() -> nvim_oxi::Result<Vec<LazyPlugin>> {
    let signs_table = table! {
        add = table!{text = icons::ADDED},
        change = table!{text = icons::CHANGED},
        delete = table!{text = icons::DELETED},
        topdelete = table!{text = icons::TOP_DELETED},
        changedelete = table!{text = icons::CHANGE_DELETED},
        untracked     = table!{text = icons::UNTRACKED}
    };

    Ok(vec![LazyPlugin::new("lewis6991/gitsigns.nvim").opts(
        table! {
            signs = &signs_table,
            signs_staged = signs_table,
            numhl = true,
            attach_to_untracked = true
        },
    )])
}
