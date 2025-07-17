mod lazy;
mod lsp;
mod macros;

#[nvim_oxi::plugin]
fn config() -> nvim_oxi::Result<()> {
    lazy::setup_lazy()?;
    lsp::setup_lsp()?;
    Ok(())
}
