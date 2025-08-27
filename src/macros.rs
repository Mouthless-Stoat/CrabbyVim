//! Contains helper macro for embeding lua.

#[macro_export]
/// Macro to help generate lua table with rust value. For a pure lua version that use lua syntax
/// see [`lua_table`](crate::lua_table)
///
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
///
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
///
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

/// Macro to help generate a lua function from rust closure with arguments. For a version that
/// doesn't need arguments use [`func`] instead.
///
/// # Examples
/// ```rust
/// let add = func_args!{
///     |a: i32, b:i32| -> i32 {
///         a + b
///     }
/// }
/// ```
#[macro_export]
macro_rules! func_args {
    (|$($param:ident: $param_ty:ty),*| -> $ret_ty:ty $body:block) => {
        {
            nvim_oxi::mlua::lua().create_function(|_lua, ($($param)*): ($($param_ty)*)| -> mlua::Result<$ret_ty> {
                fn f($($param: $param_ty)*) -> nvim_oxi::Result<$ret_ty> {
                    Ok($body)
                }
                Ok(f($($param)*).expect("can't run rust function"))
            })?
        }
    };
}

/// Macro to help generate a lua function from rust closure with no arguments. For a version that
/// allow arguments use [`func_args`] instead.
///
/// # Examples
///```rust
/// let func = func! {
///     || -> bool {
///         require("snacks")?.get::<Table>("git")?.call_function::<bool>("get_root", ())?
///     }
/// },
///```
#[macro_export]
macro_rules! func {
    (|| -> $ret_ty:ty $body:block) => {{
        nvim_oxi::mlua::lua().create_function(|_lua, ()| -> mlua::Result<$ret_ty> {
            fn f() -> nvim_oxi::Result<$ret_ty> {
                Ok($body)
            }
            Ok(f().expect("can't run rust function"))
        })?
    }};
}

/// Macro to help make a array like table in lua.
/// # Examples
/// ```rust
/// let tables = lua_vec![1, 2, 3, 4, "hello", "world"]
/// ```
#[macro_export]
macro_rules! lua_vec {
    ( $($item:expr),* ) => {
        {
            let tbl = table! {};
            $(tbl.push($item)?;)*
            tbl
        }
    };
}
