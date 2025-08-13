use crate::icons;
use crate::lazy::{LazyPlugin, LazyVersion};
use crate::table;

plugin! {
    blink_cmp with highlights;
    ---
    LazyPlugin::new("mason-org/mason.nvim").opts(table! {
        ui = table! {
            icons = table!{
                package_installed = icons::INSTALLED,
                package_pending = icons::PENDING,
                package_uninstalled = icons::UNINSTALLED
            }
        }
    });
}
