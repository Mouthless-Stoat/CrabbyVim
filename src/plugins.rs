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
}

#[macro_export]
macro_rules! add_plugins {
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
            let mut vec = vec![];
            $(
                vec.extend($plugin()?);
                $(paste::paste!([<$plugin _ $highlight>]()?;);)?
            )*
            $(
                vec.push($expr);
            )*
            Ok(vec)
        }
    };
}

pub type Plugins = nvim_oxi::Result<Vec<crate::lazy::LazyPlugin>>;

add_plugins! {
    snacks;
    conform;
    gitsigns;
    lsp;
}
