use crate::lazy::{LazyPlugin, LazyVersion};
use crate::lua_table;

add_plugins! {
    blink_cmp;
    ---
    LazyPlugin::new("L3MON4D3/LuaSnip").version(LazyVersion::Semver("v2.*"));
    LazyPlugin::new("mason-org/mason.nvim").opts(lua_table! {
        ui = {
            icons = {
                package_installed = "",
                package_pending = "",
                package_uninstalled = ""
            }
        }
    });
}
