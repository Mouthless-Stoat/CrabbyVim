macro_rules! mini {
    ($name:ident) => {
        $crate::lazy::LazyPlugin::new(concat!("echasnovski/mini.", stringify!($name)))
    };
}

plugin! {
    mini!(ai);
    mini!(move);
    mini!(pairs);
}
