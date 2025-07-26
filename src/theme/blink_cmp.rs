use crate::theme::{Color, HighlightOpt, set_hl};

#[rustfmt::skip]
pub fn configure_highlight() -> nvim_oxi::Result<()> {
    use crate::theme::syntax::SyntaxGroup::*;
    use Color::*;

    set_hl("Pmenu", HighlightOpt::with_bg(Bg1))?;
    set_hl("PmenuSel", HighlightOpt::with_bg(Bg2))?;
    set_hl("PmenuSbar", HighlightOpt::with_bg(Bg3))?;
    set_hl("PmenuThumb", HighlightOpt::with_bg(White))?;

    set_hl("BlinkCmpLabelDeprecated", HighlightOpt::default().strike())?;
    set_hl("BlinkCmpLabelMatch", HighlightOpt::with_fg(Blue).bold())?;

    set_hl_kind("Text", HighlightOpt::with_fg(Green))?;
    set_hl_kind("Method", HighlightOpt::link(Function))?;
    set_hl_kind("Function", HighlightOpt::link(Function))?;

    set_hl_kind("Constructor", HighlightOpt::link(Function))?;

    set_hl_kind("Field", HighlightOpt::link(Member))?;
    set_hl_kind("Variable", HighlightOpt::link(Identifier))?;
    set_hl_kind("Property", HighlightOpt::link(Member))?;

    set_hl_kind("Class", HighlightOpt::link(Structure))?;
    set_hl_kind("Interface", HighlightOpt::link(Structure))?;
    set_hl_kind("Struct", HighlightOpt::link(Structure))?;

    set_hl_kind("Module", HighlightOpt::link(Type))?;

    set_hl_kind("Unit", HighlightOpt::with_fg(Blue))?;
    set_hl_kind("Value", HighlightOpt::with_fg(Green))?;

    set_hl_kind("Enum", HighlightOpt::link(Structure))?;
    set_hl_kind("EnumMember", HighlightOpt::link(Structure))?;

    set_hl_kind("Keyword", HighlightOpt::link(Keyword))?;
    set_hl_kind("Constant", HighlightOpt::link(Constant))?;

    set_hl_kind("Snippet", HighlightOpt::with_fg(Blue))?;
    set_hl_kind("Color", HighlightOpt::with_fg(Pink))?;
    set_hl_kind("File", HighlightOpt::with_fg(Blue))?;
    set_hl_kind("Reference", HighlightOpt::with_fg(Purple))?;
    set_hl_kind("Folder", HighlightOpt::with_fg(Yellow))?;
    set_hl_kind("Event", HighlightOpt::with_fg(Green))?;
    set_hl_kind("Operator", HighlightOpt::link(Operator))?;
    set_hl_kind("TypeParameter", HighlightOpt::link(Type))?;

    Ok(())
}

pub fn set_hl_kind(kind: &str, opt: HighlightOpt) -> nvim_oxi::Result<()> {
    set_hl(format!("BlinkCmpKind{kind}").as_str(), opt)?;
    Ok(())
}
