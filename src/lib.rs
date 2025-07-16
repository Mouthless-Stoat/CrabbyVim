mod lazy;

#[nvim_oxi::plugin]
fn config() -> nvim_oxi::Result<()> {
    lazy::setup_lazy()?;
    Ok(())
}
