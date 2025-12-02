use crate::{
    Mode,
    keymaps::set_key,
    lazy::{LazyKey, LazyLoad},
    lua_table, require_setup,
};

macro_rules! mini {
    ($name:ident) => {
        $crate::lazy::LazyPlugin::new(concat!("nvim-mini/mini.", stringify!($name)))
    };
}

plugin! {
    mini!(ai).opts(lua_table!{
        search_method = "cover"
    }).lazy_load(
        LazyLoad::new(true)
            .add_key(LazyKey::new("a").modes(Mode::nvo()))
            .add_key(LazyKey::new("i").modes(Mode::nvo()))
    );
    mini!(move).lazy_load(
        LazyLoad::new(true)
            .add_key(LazyKey::new("<M-h>").modes(&[Mode::Normal, Mode::Visual]))
            .add_key(LazyKey::new("<M-j>").modes(&[Mode::Normal, Mode::Visual]))
            .add_key(LazyKey::new("<M-k>").modes(&[Mode::Normal, Mode::Visual]))
            .add_key(LazyKey::new("<M-l>").modes(&[Mode::Normal, Mode::Visual]))
    );
    mini!(pairs).lazy_load(LazyLoad::new(true).events(&["InsertEnter *.*"]));

    mini!(surround).lazy_load(LazyLoad::new(true).add_key("s")).opts(
            lua_table! { search_method = "cover", n_lines = 50 }
    );
}
