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
