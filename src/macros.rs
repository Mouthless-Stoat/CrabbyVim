#[macro_export]
macro_rules! lua_table {
    ($($key:ident = $value:expr),*) => {
        {
            let tbl = nvim_oxi::mlua::lua().create_table()?;

            $(tbl.set(stringify!($key), $value)?;)*

            tbl
        }
    }
}
