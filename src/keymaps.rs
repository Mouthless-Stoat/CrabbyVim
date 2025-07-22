use crate::Mode;

pub fn configure_keymaps() -> nvim_oxi::Result<()> {
    use Action::*;
    use Mode::*;

    set_key(&[Terminal], "<esc>", "<C-\\><C-n>")?;
    set_key(&[Normal], "Q", "<nop>")?;

    set_key(&[Normal], "<C-d>", "<C-d>zz")?;
    set_key(&[Normal], "<C-u>", "<C-u>zz")?;

    set_key(&[Visual], ">", ">gv")?;
    set_key(&[Visual], "<", "<gv")?;

    set_key(&[Insert], "<C-Right>", "<esc>ea")?;
    set_key(&[Insert], "<C-Left>", "<esc>ba")?;
    set_key(&[Insert], "<S-C-Right>", "<esc>Ea")?;
    set_key(&[Insert], "<S-C-Left>", "<esc>Ba")?;

    set_key(&[Normal], "k", "gk")?;
    set_key(&[Normal], "j", "gj")?;

    set_key(&[Normal], "+", "\"+")?;

    set_key(&[Normal], "H", "^")?;
    set_key(&[Normal], "L", "$")?;

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
                Ok(_) => (),
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
