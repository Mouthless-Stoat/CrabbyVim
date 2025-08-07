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
}

macro_rules! add_plugins {
    ($($plugin:ident $($with:ident $highlight:ident)?;)*) => {
        $(plugin!($plugin $($with $highlight)?);)*
        pub fn plugins() -> nvim_oxi::Result<Vec<LazyPlugin>> {
            let mut vec = vec![];
            $(
                vec.extend($plugin()?);
                $(paste::paste!(crate::theme::configure_highlights([<$plugin _ $highlight>]())?;);)?
            )*
            Ok(vec)
        }
    };
}

add_plugins! {
    snacks;
    conform;
    gitsigns;
}
