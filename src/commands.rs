//! Configure user command and export helper to create new custom user command.

pub(crate) fn configure() -> nvim_oxi::Result<()> {
    create_command(
        "Move the working directory to the current file",
        "Here",
        "cd %:h",
    )?;

    create_command(
        "Create a new scratch buffer",
        "Scratch",
        "enew | setlocal buftype=nofile bufhidden=wipe nobuflisted noswapfile",
    )?;

    create_command("Common typo", "Q", "q")?;

    Ok(())
}

/// Helper to create a new custom user command with `desc` description, execute using `name` that
/// call `cmd` upon invocation.
/// # Examples
/// ```rust
/// create_command(
///     "Create a new scratch buffer",
///     "Scratch",
///     "enew | setlocal buftype=nofile bufhidden=wipe nobuflisted noswapfile",
/// )?;
///
/// create_command(
///     "Create a new scratch buffer",
///     "Scratch",
///     "enew | setlocal buftype=nofile bufhidden=wipe nobuflisted noswapfile",
/// )?;
///
/// create_command("Common typo", "Q", "q")?;
/// ```
pub fn create_command<T>(desc: &'static str, name: &'static str, cmd: T) -> nvim_oxi::Result<()>
where
    T: nvim_oxi::api::StringOrFunction<nvim_oxi::api::types::CommandArgs, ()>,
{
    let mut opts = nvim_oxi::api::opts::CreateCommandOpts::builder();
    opts.desc(desc);

    nvim_oxi::api::create_user_command(name, cmd, &opts.build())?;
    Ok(())
}
