use crate::lazy::LazyLoad;

macro_rules! mini {
    ($name:ident) => {
        $crate::lazy::LazyPlugin::new(concat!("echasnovski/mini.", stringify!($name)))
    };
}

plugin! {
    mini!(ai);
    mini!(move).lazy_load(
        LazyLoad::new(true)
            .add_key("<M-h>")
            .add_key("<M-j>")
            .add_key("<M-k>")
            .add_key("<M-l>")
    );
    mini!(pairs).lazy_load(LazyLoad::new(true).events(&["InsertEnter *.*"]));
}
