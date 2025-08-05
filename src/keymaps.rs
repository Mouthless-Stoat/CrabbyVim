use mlua::IntoLua;

use crate::Mode;

#[rustfmt::skip]
pub fn configure() -> nvim_oxi::Result<()> {
    use Mode::*;

    nvim_oxi::api::set_var("mapleader", " ")?;

    set_key(&[Terminal], "<esc>", "<C-\\><C-n>")?; // escape in terminal mode

    set_key(&[Normal], "Q", "<nop>")?; // Eat shit and die

    // center when paging
    set_key(&[Normal], "<C-d>", "<C-d>zz")?;
    set_key(&[Normal], "<C-u>", "<C-u>zz")?;

    // keep selection when indenting
    set_key(&[Visual], ">", ">gv")?;
    set_key(&[Visual], "<", "<gv")?;

    // word movement in insert mode
    set_key(&[Insert], "<C-Right>", "<esc>ea")?;
    set_key(&[Insert], "<C-Left>", "<esc>ba")?;
    set_key(&[Insert], "<S-C-Right>", "<esc>Ea")?;
    set_key(&[Insert], "<S-C-Left>", "<esc>Ba")?;

    // movement across wrap line
    set_key(&[Normal], "k", "gk")?;
    set_key(&[Normal], "j", "gj")?;

    // easier bind pasting from system clip
    set_key(&[Normal], "+", "\"+")?;

    // keybind to go to first and last 
    set_key(&[Normal], "H", "^")?;
    set_key(&[Normal], "L", "$")?;

    // split creation and movement
    set_key(&[Normal], "<C-h>", "<C-w>h")?;
    set_key(&[Normal], "<C-j>", "<C-w>j")?;
    set_key(&[Normal], "<C-k>", "<C-w>k")?;
    set_key(&[Normal], "<C-l>", "<C-w>l")?;

    set_key_desc("Make new split left", &[Normal], "<Leader>h", "<cmd>top vnew<cr>")?;
    set_key_desc("Make new split down", &[Normal], "<Leader>j", "<cmd>bot new<cr>")?;
    set_key_desc("Make new split up", &[Normal], "<Leader>k", "<cmd>top new<cr>")?;
    set_key_desc("Make new split right", &[Normal], "<Leader>l", "<cmd>bot vnew<cr>")?;

    Ok(())
}

pub enum Action {
    Map(&'static str),
    Fn(Box<dyn FnMut() -> nvim_oxi::Result<()>>),
}

pub fn set_key(
    modes: &'static [Mode],
    key: &'static str,
    action: impl Into<Action>,
) -> nvim_oxi::Result<()> {
    set_key_desc("", modes, key, action)
}

pub fn set_key_desc(
    desc: &'static str,
    modes: &'static [Mode],
    key: &'static str,
    action: impl Into<Action>,
) -> nvim_oxi::Result<()> {
    let mut opts = nvim_oxi::api::opts::SetKeymapOpts::builder();

    opts.silent(true);
    opts.desc(desc);

    let mut rhs = "";
    match action.into() {
        Action::Map(key) => rhs = key,
        Action::Fn(mut fn_mut) => {
            opts.callback(move |()| match fn_mut() {
                Ok(()) => (),
                Err(err) => nvim_oxi::api::err_writeln(format!("{err}").as_str()),
            });
        }
    }

    for mode in modes {
        nvim_oxi::api::set_keymap((*mode).into(), key, rhs, &opts.build())?;
    }
    Ok(())
}

impl From<&'static str> for Action {
    fn from(val: &'static str) -> Self {
        Action::Map(val)
    }
}

impl IntoLua for Action {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        match self {
            Action::Map(str) => str.into_lua(lua),
            Action::Fn(mut fn_mut) => Ok(mlua::Value::Function(lua.create_function_mut(
                move |_, _: ()| Ok(fn_mut().expect("Cannot turn key action into lua function")),
            )?)),
        }
    }
}
