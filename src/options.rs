use crate::keymaps::{Action, set_key};
use crate::{Mode, table};
use mlua::{Function, Table};
use nvim_oxi::conversion::ToObject;

pub fn configure() -> nvim_oxi::Result<()> {
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

    set_option("list", true)?;
    set_option("listchars", "multispace:·,tab:<->")?;

    set_option("swapfile", false)?;
    set_option("backup", false)?;
    set_option(
        "undodir",
        std::path::Path::new(&std::env::var("XDG_DATA_HOME").unwrap())
            .join("nvim-data/undo")
            .to_str()
            .unwrap()
            .replace('/', "\\"),
    )?;
    set_option("undofile", true)?;

    set_option("scrolloff", 8)?;
    set_option("guifont", "CaskaydiaCove Nerd Font Mono:h10:#h-none")?;
    set_option("shell", "powershell")?;
    set_option("shellcmdflag", "-c")?;

    let yank_callback = |_args| {
        nvim_oxi::mlua::lua()
            .globals()
            .get::<Table>("vim")?
            .get::<Table>("hl")?
            .get::<Function>("on_yank")?
            .call::<bool>(table! { higroup = "Yank" })
    };

    let opts = nvim_oxi::api::opts::CreateAutocmdOpts::builder()
        .patterns(["*"])
        .callback(yank_callback)
        .build();

    nvim_oxi::api::create_autocmd(["TextYankPost"], &opts)?;

    if nvim_oxi::api::get_var::<bool>("neovide").is_ok() {
        configure_neovide()?;
    }

    Ok(())
}

fn configure_neovide() -> nvim_oxi::Result<()> {
    set_neovide_option("refresh_rate", 120)?;
    set_neovide_option("scale_factor", 1.0)?;
    set_neovide_option("cursor_animation_length", 0.08)?;
    set_neovide_option("cursor_trail_size", 0.5)?;
    set_neovide_option("scroll_animation_length", 0.5)?;

    set_key(
        &[Mode::Normal],
        "<C-=>",
        Action::Fn(Box::new(|| {
            set_neovide_option(
                "scale_factor",
                nvim_oxi::api::get_var::<f64>("neovide_scale_factor").unwrap() + 0.1,
            )
        })),
    )?;
    set_key(
        &[Mode::Normal],
        "<C-->",
        Action::Fn(Box::new(|| {
            set_neovide_option(
                "scale_factor",
                nvim_oxi::api::get_var::<f64>("neovide_scale_factor").unwrap() - 0.1,
            )
        })),
    )?;

    Ok(())
}

pub fn set_option<T: ToObject>(name: &str, value: T) -> nvim_oxi::Result<()> {
    nvim_oxi::api::set_option_value::<T>(name, value, &nvim_oxi::api::opts::OptionOpts::default())?;
    Ok(())
}

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
