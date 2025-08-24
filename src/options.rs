//! Configure and setup built in neovim options

use mlua::{Function, Table};
use nvim_oxi::conversion::{FromObject, ToObject};

use crate::{Mode, autocmds::create_autocmd, keymaps::set_key, table, vim_fn};

pub(crate) fn configure() -> nvim_oxi::Result<()> {
    set_option("number", true)?;
    set_option("relativenumber", true)?;

    set_option("ignorecase", true)?;
    set_option("smartcase", true)?;

    set_option("tabstop", 4)?;
    set_option("shiftwidth", 4)?;
    set_option("expandtab", true)?;

    set_option("updatetime", 1000)?;

    set_option("signcolumn", "yes")?;

    set_option("showmode", false)?;

    set_option("termguicolors", true)?;

    set_option("hlsearch", false)?;

    set_option("wrap", false)?;

    set_option("cursorline", true)?;
    set_option("cursorlineopt", "number")?;

    set_option("virtualedit", "block")?;

    set_option("list", true)?;
    set_option("listchars", "lead: ,multispace:·,tab:<->")?;

    set_option("swapfile", false)?;
    set_option("backup", false)?;
    set_option(
        "undodir",
        std::path::Path::new(&vim_fn::<String>("stdpath", "data")?)
            .join("undo")
            .into_os_string()
            .into_string()
            .unwrap(),
    )?;
    set_option("undofile", true)?;

    set_option("scrolloff", 8)?;
    set_option("guifont", "CaskaydiaCove Nerd Font Mono:h10:#h-none")?;

    if vim_fn("has", "win32")? {
        // god bless this soul https://www.reddit.com/r/neovim/comments/1crdv93/comment/lolujpr
        set_option("shell", "pwsh.exe")?;
        set_option(
            "shellcmdflag",
            "-NoLogo -NoProfile -ExecutionPolicy RemoteSigned -Command $PSStyle.OutputRendering = 'PlainText';",
        )?;

        set_option(
            "shellredir",
            "2>&1 | Out-File -Encoding UTF8 %s; exit $LastExitCode",
        )?;
        set_option(
            "shellpipe",
            "2>&1 | Out-File -Encoding UTF8 %s; exit $LastExitCode",
        )?;
        set_option("shellquote", "")?;
        set_option("shellxquote", "")?;
    }

    create_autocmd(&["TextYankPost"], &["*"], |_| {
        crate::vim()?
            .get::<Table>("hl")?
            .get::<Function>("on_yank")?
            .call::<bool>(table! { higroup = "Yank" })?;

        Ok(())
    })?;

    if nvim_oxi::api::get_var::<bool>("neovide").is_ok() {
        configure_neovide()?;
    }

    Ok(())
}

fn configure_neovide() -> nvim_oxi::Result<()> {
    set_neovide_option("scale_factor", 1.0)?;
    set_neovide_option("cursor_animation_length", 0.08)?;
    set_neovide_option("cursor_trail_size", 0.5)?;
    set_neovide_option("position_animation_length", 0)?;

    set_neovide_option("padding_left", 10)?;

    set_key(&[Mode::Normal], "<C-=>", || {
        set_neovide_option(
            "scale_factor",
            nvim_oxi::api::get_var::<f64>("neovide_scale_factor").unwrap() + 0.1,
        )
    })?;
    set_key(&[Mode::Normal], "<C-->", || {
        set_neovide_option(
            "scale_factor",
            nvim_oxi::api::get_var::<f64>("neovide_scale_factor").unwrap() - 0.1,
        )
    })?;

    Ok(())
}

/// Helper to set a neovim option with the given `name` to `value`.
///
/// # Examples
/// ```rust
/// set_option("number", true)?;
/// set_option("relativenumber", true)?;
///
/// set_option("ignorecase", true)?;
/// set_option("smartcase", true)?;
///
/// set_option("tabstop", 4)?;
/// set_option("shiftwidth", 4)?;
/// set_option("expandtab", true)?;
/// ```
pub fn set_option<T: ToObject>(name: &str, value: T) -> nvim_oxi::Result<()> {
    nvim_oxi::api::set_option_value::<T>(name, value, &nvim_oxi::api::opts::OptionOpts::default())?;
    Ok(())
}

/// Set a local option like window local and buffer local with the given `name` to `value`.
///
/// # Examples
/// ```rust
/// create_autocmd(&["BufEnter"], &["*.md"], |_| {
///     set_local_option("wrap", true)?;
///     set_local_option("linebreak", true)?;
///     set_local_option("spell", true)?;
///     set_local_option("breakindent", true)?;
///     set_local_option("showbreak", "| ")?;
///     Ok(())
/// })?;
/// ```
pub fn set_local_option<T: ToObject>(name: &str, value: T) -> nvim_oxi::Result<()> {
    nvim_oxi::api::set_option_value::<T>(
        name,
        value,
        &nvim_oxi::api::opts::OptionOpts::builder()
            .scope(nvim_oxi::api::opts::OptionScope::Local)
            .build(),
    )?;
    Ok(())
}

fn set_neovide_option<T: ToObject>(name: &str, value: T) -> nvim_oxi::Result<()> {
    nvim_oxi::api::set_var(format!("neovide_{name}").as_str(), value)?;
    Ok(())
}

/// Get the value for a given `name` option.
pub fn get_option<T: FromObject>(name: &str) -> nvim_oxi::Result<T> {
    Ok(nvim_oxi::api::get_option_value(
        name,
        &nvim_oxi::api::opts::OptionOpts::default(),
    )?)
}
