pub fn configure() -> nvim_oxi::Result<()> {
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
    create_command("Common quit typo", "Q", "q")?;

    Ok(())
}

fn create_command<T>(desc: &'static str, name: &'static str, cmd: T) -> nvim_oxi::Result<()>
where
    T: nvim_oxi::api::StringOrFunction<nvim_oxi::api::types::CommandArgs, ()>,
{
    let mut opts = nvim_oxi::api::opts::CreateCommandOpts::builder();
    opts.desc(desc);

    nvim_oxi::api::create_user_command(name, cmd, &opts.build())?;
    Ok(())
}
