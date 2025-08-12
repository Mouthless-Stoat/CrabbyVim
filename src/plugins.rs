use crate::lazy::LazyPlugin;

macro_rules! plugin {
    ($mod:ident) => {
        mod $mod;
        use $mod::plugins as $mod;
    };
    ($mod:ident with highlights) => {
        mod $mod;
        paste::paste!(use $mod::highlights as [<$mod _highlights>];);
        use $mod::plugins as $mod;
    };
    ($($plugin:ident $($with:ident $highlight:ident)?;)*) => {
        $(plugin!($plugin $($with $highlight)?);)*
        pub fn plugins() -> $crate::plugins::Plugins {
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
        pub fn plugins() -> $crate::plugins::Plugins {
            let mut vec = vec![$($expr.into()),*];
            $(
                vec.extend($plugin()?);
                $(paste::paste!([<$plugin _ $highlight>]()?;);)?
            )*
            Ok(vec)
        }
    };
    ($($expr:expr;)*) => {
        pub fn plugins() -> $crate::plugins::Plugins {
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
    ---
    "nvim-tree/nvim-web-devicons";
    "brianhuster/unnest.nvim";
    "wakatime/vim-wakatime";
}
