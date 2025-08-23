//! Contains helper macro for embeding lua.

#[macro_export]
/// Macro to help generate lua table with rust value. For a pure lua version that use lua syntax
/// see [`lua_table`](crate::lua_table)
/// # Example
/// ```rust
/// let a = 34;
/// let b = 35;
/// let flag = true;
/// let tbl = table! {
///     nice = a + b,
///     flag = flag
/// };
/// ```
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

/// Macro to help with embeding a pure lua table into rust code. Useful for setting the `opt` field of plugin.
/// # Example
/// ```rust
/// let mini_surround_opt = lua_table! {
///     mappings = {
///     add = "ys",
///         delete = "ds",
///         find = "",
///         find_left = "",
///         highlight = "",
///         replace = "cs",
///         update_n_lines = "",
///         suffix_last = "",
///         suffix_next = "",
///     },
///     search_method = "cover_or_next",
/// }
/// ```
#[macro_export]
macro_rules! lua_table {
    ($($tk:tt)*) => {
        nvim_oxi::mlua::lua().load(concat!("{",stringify!($($tk)*),"}")).eval::<mlua::Table>()?
    };
}

/// Macro to help with embeding a lua expression into rust code.
/// # Examples
/// ```rust
/// let lua_func = expr!{
///     return mlua::Function;
///     function(a, b) return a + b end
/// }
///
/// let lua_tbl = expr!{
///     return mlua::Table;
///     {
///         [1] = "hello world",
///         [2] = "goodbye world",
///         a = 10,
///         b = 11,
///         c = 12,
///     }
/// }
/// ```
#[macro_export]
macro_rules! expr {
    (return $ty:ty; $($tk:tt)*) => {
        nvim_oxi::mlua::lua().load(stringify!($($tk)*)).eval::<$ty>()?
    };
}
