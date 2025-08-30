#![allow(missing_docs)]

use crate::lazy::{LazyLoad, LazyPlugin};

macro_rules! plugin {
    ($mod:ident) => {
        pub mod $mod;
        use $mod::plugins as $mod;
    };
    ($mod:ident with highlights) => {
        pub mod $mod;
        paste::paste!(use $mod::highlights as [<$mod _highlights>];);
        use $mod::plugins as $mod;
    };
    ($($plugin:ident $($with:ident $highlight:ident)?;)*) => {
        $(plugin!($plugin $($with $highlight)?);)*
        pub(crate) fn plugins() -> $crate::plugins::Plugins {
            let mut vec = vec![];
            $(
                vec.extend($plugin()?);
                $(paste::paste!([<$plugin _ $highlight>]()?;);)?
            )*
            Ok(vec)
        }
    };
    ($($plugin:ident $($with:ident $highlight:ident)?;)*---$($expr:expr;)*) => {
        $(plugin!($plugin $($with $highlight)?);)*
        pub(crate) fn plugins() -> $crate::plugins::Plugins {
            let mut vec = vec![$($expr.into()),*];
            $(
                vec.extend($plugin()?);
                $(paste::paste!([<$plugin _ $highlight>]()?;);)?
            )*
            Ok(vec)
        }
    };
    ($($expr:expr;)*) => {
        pub(crate) fn plugins() -> $crate::plugins::Plugins {
            Ok(vec![$($expr.into()),*])
        }
    };
}

pub type Plugins = nvim_oxi::Result<Vec<LazyPlugin>>;

plugin! {
    snacks with highlights;
    conform;
    gitsigns with highlights;
    lsp;
    treesitter;
    fidget with highlights;
    oil;
    mini;
    delimiters with highlights;
    devicons;
    hop;
    markview with highlights;
    undotree;
    ---
    LazyPlugin::new ("wakatime/vim-wakatime").lazy_load(LazyLoad::new(true).events(&["VeryLazy"]));
    LazyPlugin::new("FilipHarald/aw-watcher-vim").callback(|_|{Ok(())}).lazy_load(LazyLoad::new(true).events(&["VeryLazy"]));
}
