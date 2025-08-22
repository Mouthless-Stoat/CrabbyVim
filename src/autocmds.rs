use nvim_oxi::api::types::LogLevel;

use crate::options::set_local_option;
use crate::{vim_fn, vim_notify};

pub fn configure() -> nvim_oxi::Result<()> {
    create_autocmd(&["BufEnter"], &["*.md"], |_| {
        set_local_option("wrap", true)?;
        set_local_option("linebreak", true)?;
        set_local_option("spell", true)?;
        set_local_option("breakindent", true)?;
        set_local_option("showbreak", "| ")?;
        Ok(())
    })?;

    create_autocmd(&["BufWritePost"], &["*"], |_| {
        vim_notify(
            format!("{} saved", vim_fn::<String>("expand", "<afile>:t")?).as_str(),
            LogLevel::Info,
        )?;
        Ok(())
    })?;

    Ok(())
}

pub fn create_autocmd<T>(
    events: &'static [&'static str],
    patterns: &'static [&'static str],
    callback: T,
) -> nvim_oxi::Result<()>
where
    T: Fn(nvim_oxi::api::types::AutocmdCallbackArgs) -> nvim_oxi::Result<()> + 'static,
{
    nvim_oxi::api::create_autocmd(
        events.iter().copied(),
        &nvim_oxi::api::opts::CreateAutocmdOpts::builder()
            .patterns(patterns.iter().copied())
            .callback(move |args| -> nvim_oxi::Result<bool> {
                callback(args)?;
                Ok(false)
            })
            .build(),
    )?;
    Ok(())
}
