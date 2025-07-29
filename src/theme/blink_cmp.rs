use crate::theme::{Color, HighlightOpt, set_hl};

#[rustfmt::skip]
pub fn configure_highlight() -> nvim_oxi::Result<()> {
    use Color::*;
    use KindHighlight::*;

    set_hl("PinkKind", HighlightOpt::with_bg(Pink).fg(Bg1).bold())?;
    set_hl("RedKind", HighlightOpt::with_bg(Red).fg(Bg1).bold())?;
    set_hl("OrangeKind", HighlightOpt::with_bg(Orange).fg(Bg1).bold())?;
    set_hl("YellowKind", HighlightOpt::with_bg(Yellow).fg(Bg1).bold())?;
    set_hl("GreenKind", HighlightOpt::with_bg(Green).fg(Bg1).bold())?;
    set_hl("CyanKind", HighlightOpt::with_bg(Cyan).fg(Bg1).bold())?;
    set_hl("BlueKind", HighlightOpt::with_bg(Blue).fg(Bg1).bold())?;
    set_hl("PurpleKind", HighlightOpt::with_bg(Purple).fg(Bg1).bold())?;

    set_hl("Pmenu", HighlightOpt::with_bg(Bg1))?;
    set_hl("PmenuSel", HighlightOpt::with_bg(Bg2))?;
    set_hl("PmenuSbar", HighlightOpt::with_bg(Bg3))?;
    set_hl("PmenuThumb", HighlightOpt::with_bg(White))?;

    set_blink_hl("LabelDeprecated", HighlightOpt::default().strike())?;
    set_blink_hl("LabelMatch", HighlightOpt::with_fg(Blue).bold())?;

    set_hl_kind("Text",GreenKind)?;
    set_hl_kind("Method", BlueKind)?;
    set_hl_kind("Function", BlueKind)?;

    set_hl_kind("Constructor", YellowKind)?;

    set_hl_kind("Field", CyanKind)?;
    set_hl_kind("Variable", RedKind)?;
    set_hl_kind("Property", CyanKind)?;

    set_hl_kind("Class", YellowKind)?;
    set_hl_kind("Interface", OrangeKind)?;
    set_hl_kind("Struct", YellowKind)?;

    set_hl_kind("Module", OrangeKind)?;

    set_hl_kind("Unit", BlueKind)?;
    set_hl_kind("Value", GreenKind)?;

    set_hl_kind("Enum", YellowKind)?;
    set_hl_kind("EnumMember", YellowKind)?;

    set_hl_kind("Keyword", PurpleKind)?;
    set_hl_kind("Constant", OrangeKind)?;

    set_hl_kind("Snippet", PurpleKind)?;
    set_hl_kind("Color", PinkKind)?;
    set_hl_kind("File", CyanKind)?;
    set_hl_kind("Reference", PurpleKind)?;
    set_hl_kind("Folder", YellowKind)?;
    set_hl_kind("Event", GreenKind)?;
    set_hl_kind("Operator", CyanKind)?;
    set_hl_kind("TypeParameter", OrangeKind)?;

    Ok(())
}

#[derive(Clone, Copy)]
#[allow(clippy::enum_variant_names)]
enum KindHighlight {
    PinkKind,
    RedKind,
    OrangeKind,
    YellowKind,
    GreenKind,
    CyanKind,
    BlueKind,
    PurpleKind,
}

fn set_hl_kind(kind: &str, hl: KindHighlight) -> nvim_oxi::Result<()> {
    set_blink_hl(
        format!("Kind{kind}").as_str(),
        HighlightOpt::link(match hl {
            KindHighlight::PinkKind => "PinkKind",
            KindHighlight::RedKind => "RedKind",
            KindHighlight::OrangeKind => "OrangeKind",
            KindHighlight::YellowKind => "YellowKind",
            KindHighlight::GreenKind => "GreenKind",
            KindHighlight::CyanKind => "CyanKind",
            KindHighlight::BlueKind => "BlueKind",
            KindHighlight::PurpleKind => "PurpleKind",
        }),
    )?;
    Ok(())
}

fn set_blink_hl(hl: &str, opt: HighlightOpt) -> nvim_oxi::Result<()> {
    set_hl(format!("BlinkCmp{hl}").as_str(), opt)?;
    Ok(())
}
