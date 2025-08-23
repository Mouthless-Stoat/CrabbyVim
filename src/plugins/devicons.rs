use mlua::ObjectLike;

use crate::{require, table};

use super::Plugins;

#[allow(clippy::unnecessary_wraps)]
pub(crate) fn plugins() -> Plugins {
    Ok(vec!["nvim-tree/nvim-web-devicons".into()])
}

pub fn get_icon_by_filetype(filetype: String) -> nvim_oxi::Result<String> {
    Ok(require("nvim-web-devicons")?.call_function(
        "get_icon_by_filetype",
        (filetype, table! { default = true }),
    )?)
}

pub fn get_icon(file: &str) -> nvim_oxi::Result<(String, String)> {
    let (name, ext) = {
        let mut t = file.split('.');
        match (t.next(), t.next()) {
            (Some(name), None) => (name, ""),
            (Some(name), Some(ext)) => (name, ext),
            (None, None | Some(_)) => unreachable!(),
        }
    };

    Ok(require("nvim-web-devicons")?
        .call_function("get_icon", (name, ext, table! { default=true }))?)
}
