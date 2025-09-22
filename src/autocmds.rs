//! Configure autocmd and export helper to create new autocmd easier.
use nvim_oxi::api::types::LogLevel;

use crate::{options::set_local_option, vim_fn, vim_notify};

pub(crate) fn configure() -> nvim_oxi::Result<()> {
    create_autocmd(&["BufEnter"], &["*.md", "*.typ"], |_| {
        set_local_option("wrap", true)?;
        set_local_option("linebreak", true)?;
        set_local_option("spell", true)?;
        set_local_option("breakindent", true)?;
        set_local_option("showbreak", "│ ")?;
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

/// Helper to create an autocmd that fire on `events` that match `patterns` which then call
/// `callback`. This `callback` does not need to return a `bool` because a one shot autocmd is
/// rarely useful in this context.
///
/// For a oneshot version see [`create_autocmd_oneshot`].
///
/// # Examples
///```rust
/// create_autocmd(&["BufEnter"], &["*.md"], |_| {
///     set_local_option("wrap", true)?;
///     set_local_option("linebreak", true)?;
///     set_local_option("spell", true)?;
///     set_local_option("breakindent", true)?;
///     set_local_option("showbreak", "| ")?;
///     Ok(())
/// })?;
///
/// create_autocmd(&["BufWritePost"], &["*"], |_| {
///     vim_notify(
///         format!("{} saved", vim_fn::<String>("expand", "<afile>:t")?).as_str(),
///         LogLevel::Info,
///     )?;
///     Ok(())
/// })?;
/// ```
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

/// Helper to create an autocmd that fire on `events` that match `patterns` which then call
/// `callback`. This `callback` will only be call once, and this autocmd will be delete.
///
/// For a repeateable version of this see [`create_autocmd`].
///
/// # Examples
/// ```rust
/// create_autocmd_oneshot(&["User"], &["VeryLazy"], |_| {
///     set_option("statusline", "%!v:lua.statusline()")?;
///     set_option("winbar", "%{%v:lua.winbar()%}")?;
///     create_autocmd_cmd(&["User"], &["GitSignsUpdate"], "redrawstatus!")?;
///     create_autocmd_cmd(&["DiagnosticChanged"], &["*"], "redrawstatus!")?;
//     Ok(())
/// })?;
/// ```
pub fn create_autocmd_oneshot<T>(
    event: &'static str,
    patterns: &'static [&'static str],
    callback: T,
) -> nvim_oxi::Result<()>
where
    T: Fn(nvim_oxi::api::types::AutocmdCallbackArgs) -> nvim_oxi::Result<()> + 'static,
{
    nvim_oxi::api::create_autocmd(
        [event],
        &nvim_oxi::api::opts::CreateAutocmdOpts::builder()
            .patterns(patterns.iter().copied())
            .callback(nvim_oxi::Function::from_fn_once(
                move |args| -> nvim_oxi::Result<bool> {
                    callback(args)?;
                    Ok(true)
                },
            ))
            .build(),
    )?;
    Ok(())
}

/// Helper to create an autocmd that fire on `events` that match `patterns` which then execute a
/// vim comment as a callback.
///
/// # Examples
/// ```rust
/// create_autocmd_cmd(&["User"], &["GitSignsUpdate"], "redrawstatus!")?;
/// ```
pub fn create_autocmd_cmd(
    events: &'static [&'static str],
    patterns: &'static [&'static str],
    cmd: &'static str,
) -> nvim_oxi::Result<()> {
    nvim_oxi::api::create_autocmd(
        events.iter().copied(),
        &nvim_oxi::api::opts::CreateAutocmdOpts::builder()
            .patterns(patterns.iter().copied())
            .command(cmd)
            .build(),
    )?;
    Ok(())
}
