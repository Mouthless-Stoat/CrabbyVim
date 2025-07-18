mod lazy;
mod lsp;
mod macros;
mod options;

#[nvim_oxi::plugin]
fn config() -> nvim_oxi::Result<()> {
    lazy::setup_lazy()?;
    lsp::setup_lsp()?;
    options::configure_options()?;
    Ok(())
}
