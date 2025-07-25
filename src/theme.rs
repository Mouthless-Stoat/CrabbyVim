mod blink_cmp;
mod syntax;

pub fn configure() -> nvim_oxi::Result<()> {
    configure_highlight()?;
    syntax::configure_highlight()?;
    blink_cmp::configure_highlight()?;

    Ok(())
}

#[rustfmt::skip]
fn configure_highlight() -> nvim_oxi::Result<()> {
    use Color::*;
    set_hl("Normal", HighlightOpt::default().fg(White).bg(Bg0))?;

    set_hl("NCursor", HighlightOpt::default().bg(Blue))?;
    set_hl("ICursor", HighlightOpt::default().bg(Green))?;
    set_hl("VCursor", HighlightOpt::default().bg(Purple))?;
    set_hl("CCursor", HighlightOpt::default().bg(Yellow))?;
    set_hl("RCursor", HighlightOpt::default().bg(Red))?;

    set_hl("ErrorMsg", HighlightOpt::default().fg(Red))?;
    set_hl("MoreMsg", HighlightOpt::default().fg(Blue))?;
    set_hl("WarningMsg", HighlightOpt::default().fg(Yellow))?;
    set_hl("Question", HighlightOpt::default().fg(Green))?;

    set_hl("IncSearch", HighlightOpt::default().fg(Bg0).bg(Yellow))?;
    set_hl("Subsitube", HighlightOpt::default().fg(Bg0).bg(Green))?;

    set_hl("Yank", HighlightOpt::default().reverse())?;

    set_hl("Visual", HighlightOpt::default().fg(Bg0).bg(Purple))?;
    set_hl("EndOfBuffer", HighlightOpt::default().fg(Bg0))?;

    set_hl("LineNr", HighlightOpt::default().fg(Gray).bg(Bg1))?;
    set_hl("CursorLineNr", HighlightOpt::default().fg(Blue).bg(Bg0))?;

    set_hl("DiffAdd", HighlightOpt::default().fg(Green).bg(Bg0).bold().italic())?;
    set_hl("DiffChange", HighlightOpt::default().fg(Yellow).bg(Bg0).bold().italic())?;
    set_hl("DiffDelete", HighlightOpt::default().fg(Red).bg(Bg0).bold().italic(),)?;
    set_hl("DiffText", HighlightOpt::default().fg(Blue).bg(Bg0).bold().italic())?;

    set_hl("Changed", HighlightOpt::default().fg(Yellow).bold())?;
    set_hl("Added", HighlightOpt::default().fg(Green).bold())?;
    set_hl("Removed", HighlightOpt::default().fg(Red).bold())?;

    set_hl("WinSeparator", HighlightOpt::default().fg(Blue).bg(Bg1))?;

    set_hl("MatchParen", HighlightOpt::default().fg(Blue).bg(Bg1))?;

    set_hl("MsgArea", HighlightOpt::default().fg(Yellow).bg(Bg0))?;

    set_hl("SpellBad", HighlightOpt::default().fg(Red))?;
    set_hl("SpellCap", HighlightOpt::default().fg(Blue))?;
    set_hl("SpellLocal", HighlightOpt::default().fg(Yellow))?;
    set_hl("SpellRare", HighlightOpt::default().fg(Green))?;

    set_hl("DiagnosticError", HighlightOpt::default().fg(Red))?;
    set_hl("DiagnosticWarn", HighlightOpt::default().fg(Yellow))?;
    set_hl("DiagnosticInfo", HighlightOpt::default().fg(Blue))?;
    set_hl("DiagnosticHint", HighlightOpt::default().fg(Purple))?;
    set_hl("DiagnosticOk", HighlightOpt::default().fg(Green))?;

    set_hl("DiagnosticUnderlineError", HighlightOpt::default().fg(Red).underline())?;
    set_hl("DiagnosticUnderlineWarn", HighlightOpt::default().fg(Yellow).underline())?;
    set_hl("DiagnosticUnderlineInfo", HighlightOpt::default().fg(Blue).underline())?;
    set_hl("DiagnosticUnderlineHint", HighlightOpt::default().fg(Purple).underline())?;
    set_hl("DiagnosticUnderlineOk", HighlightOpt::default().fg(Green).underline())?;

    Ok(())
}

// Macro abuse? Yessir
// It just annoying to type pub const over and over and over and over...
macro_rules! colors {
    ($($name:ident = $value:literal;)*) => {
        #[derive(Clone, Copy)]
        pub enum Color {
            $($name,)*
        }
        impl Color {
            fn to_str(self) -> &'static str {
                match self {
                    $(Self::$name => $value,)*
                }
            }
        }
    }
}

colors! {
    Pink = "#ff4f9b";
    Red = "#f65866";
    Orange = "#fa9534";
    Yellow = "#efbd5d";
    Green = "#8bcd5b";
    Cyan = "#00b8b8";
    Blue = "#41a7fc";
    Purple = "#c75ae8";

    White = "#829bcd";
    Gray = "#68687a";

    Bg0 = "#1F1F28"; // default bg
    Bg1 = "#2A2A37";
    Bg2 = "#363646";
    Bg3 = "#454556"; // lightest
}

// Not using SetHighlightOpts by nvim_oxi because it is too complex with too many feature that we
// never use
#[derive(Clone, Copy, Default)]
pub struct HighlightOpt {
    fg: Option<Color>,
    bg: Option<Color>,
    underline: bool,
    bold: bool,
    italic: bool,
    reverse: bool,
    link: Option<&'static str>,
    strike: bool,
}

impl HighlightOpt {
    fn link(hl: impl Into<&'static str>) -> Self {
        let mut t = Self::default();
        t.link = Some(hl.into());
        t
    }

    fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }
    fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }
    fn underline(mut self) -> Self {
        self.underline = true;
        self
    }
    fn bold(mut self) -> Self {
        self.bold = true;
        self
    }
    fn italic(mut self) -> Self {
        self.italic = true;
        self
    }
    fn reverse(mut self) -> Self {
        self.reverse = true;
        self
    }
    fn strike(mut self) -> Self {
        self.strike = true;
        self
    }
}

pub fn set_hl<'a>(name: impl Into<&'a str>, opt: HighlightOpt) -> nvim_oxi::Result<()> {
    let mut opt_builder = nvim_oxi::api::opts::SetHighlightOpts::builder();

    if let Some(link) = opt.link {
        opt_builder.link(link);
    } else {
        if let Some(fg) = opt.fg {
            opt_builder.foreground(fg.to_str());
        }
        if let Some(bg) = opt.bg {
            opt_builder.background(bg.to_str());
        }

        opt_builder.underline(opt.underline);
        opt_builder.bold(opt.bold);
        opt_builder.italic(opt.italic);
    }
    opt_builder.force(true);

    nvim_oxi::api::set_hl(0, name.into(), &opt_builder.build())?;

    Ok(())
}
