use crate::keymaps::set_key;
use crate::lazy::LazyLoad;
use crate::{lua_table, require_setup};

macro_rules! mini {
    ($name:ident) => {
        $crate::lazy::LazyPlugin::new(concat!("echasnovski/mini.", stringify!($name)))
    };
}

plugin! {
    mini!(ai).opts(lua_table!{
        search_method = "cover"
    });
    mini!(move).lazy_load(
        LazyLoad::new(true)
            .add_key("<M-h>")
            .add_key("<M-j>")
            .add_key("<M-k>")
            .add_key("<M-l>")
    );
    mini!(pairs).lazy_load(LazyLoad::new(true).events(&["InsertEnter *.*"]));

    // follow mini.surround doc for similar setup to tpope/vim-surround. Hitting a twice for
    // arround motion feel strange
    mini!(surround)
        .lazy_load(
            LazyLoad::new(true)
                .add_key("ys")
                .add_key("ds")
                .add_key("cs")
                .add_key("V")
        ).opts(lua_table! {
            mappings = {
            add = "ys",
                delete = "ds",
                find = "",
                find_left = "",
                highlight = "",
                replace = "cs",
                update_n_lines = "",
                suffix_last = "",
                suffix_next = "",
            },
            search_method = "cover_or_next",
        })
        .callback(|opts| {
            use crate::Mode::*;

            require_setup("mini.surround", opts)?;

            nvim_oxi::api::del_keymap(Visual.into(), "ys")?;
            set_key(&[Visual], "S", "[[:<C-u>lua MiniSurround.add('visual')<CR>]]")?;

            set_key(&[Normal], "yss", "ys_")?;

            Ok(())
        });
}
