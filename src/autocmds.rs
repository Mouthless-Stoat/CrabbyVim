use crate::options::set_local_option;

pub fn configure() -> nvim_oxi::Result<()> {
    create_autocmd(&["BufEnter"], &["*.md"], |_| {
        set_local_option("wrap", true)?;
        set_local_option("linebreak", true)?;
        set_local_option("spell", true)?;
        set_local_option("breakindent", true)?;
        set_local_option("showbreak", "| ")?;
        Ok(false)
    })?;

    Ok(())
}

pub fn create_autocmd<T>(
    events: &'static [&'static str],
    patterns: &'static [&'static str],
    callback: T,
) -> nvim_oxi::Result<()>
where
    T: Fn(nvim_oxi::api::types::AutocmdCallbackArgs) -> nvim_oxi::Result<bool> + 'static,
{
    let mut opts = nvim_oxi::api::opts::CreateAutocmdOpts::builder();

    opts.patterns(patterns.into_iter().map(|s| *s));
    opts.callback(callback);

    nvim_oxi::api::create_autocmd(events.into_iter().map(|s| *s), &opts.build())?;

    Ok(())
}
