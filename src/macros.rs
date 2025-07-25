#[macro_export]
macro_rules! table {
    ($($key:ident = $value:expr),*) => {
        {
            let tbl = nvim_oxi::mlua::lua().create_table()?;

            $(tbl.set(stringify!($key), $value)?;)*

            tbl
        }
    };
    ($([$key:expr] = $value:expr),*) => {
        {
            let tbl = nvim_oxi::mlua::lua().create_table()?;

            $(tbl.set($key, $value)?;)*

            tbl
        }
    }
}

/// Straight up embed a table with lua code
#[macro_export]
macro_rules! lua_table {
    ($($tk:tt)*) => {
        nvim_oxi::mlua::lua().load(concat!("{",stringify!($($tk)*),"}")).eval::<mlua::Table>()?
    };
}

#[macro_export]
macro_rules! expr {
    (return $ty:ty; $($tk:tt)*) => {
        nvim_oxi::mlua::lua().load(stringify!($($tk)*)).eval::<mlua::$ty>()?
    };
}
