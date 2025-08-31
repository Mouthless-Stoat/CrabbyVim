use super::{Color::*, HighlightOpt, set_hl};

pub(crate) fn highlights() -> nvim_oxi::Result<()> {
    set_markup_hl("raw", HighlightOpt::with_fg(Orange).bg(Bg1))?;
    set_markup_hl("link", HighlightOpt::with_fg(Blue).bold())?;
    set_markup_hl("quote", HighlightOpt::with_fg(Gray))?;

    set_markup_hl("list.checked", HighlightOpt::with_fg(Green))?;
    set_markup_hl("list.unchecked.", HighlightOpt::with_fg(Red))?;

    set_markup_hl("heading.1", HighlightOpt::with_fg(Blue))?;
    set_markup_hl("heading.2", HighlightOpt::with_fg(Green))?;
    set_markup_hl("heading.3", HighlightOpt::with_fg(Yellow))?;
    set_markup_hl("heading.4", HighlightOpt::with_fg(Orange))?;
    set_markup_hl("heading.5", HighlightOpt::with_fg(Red))?;
    set_markup_hl("heading.6", HighlightOpt::with_fg(Purple))?;

    Ok(())
}

fn set_markup_hl(hl: &str, opt: HighlightOpt) -> nvim_oxi::Result<()> {
    set_hl(format!("@markup.{hl}").as_str(), opt)?;
    Ok(())
}
