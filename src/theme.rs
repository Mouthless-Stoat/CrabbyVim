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
    set_hl("Normal", HighlightOpt::with_fg(White).bg(Bg0))?;
    set_hl("NormalFloat", HighlightOpt::with_fg(White).bg(Bg1))?;

    set_hl("NCursor", HighlightOpt::with_bg(Blue))?;
    set_hl("ICursor", HighlightOpt::with_bg(Green))?;
    set_hl("VCursor", HighlightOpt::with_bg(Purple))?;
    set_hl("CCursor", HighlightOpt::with_bg(Yellow))?;
    set_hl("RCursor", HighlightOpt::with_bg(Red))?;

    set_hl("ErrorMsg", HighlightOpt::with_fg(Red))?;
    set_hl("MoreMsg", HighlightOpt::with_fg(Blue))?;
    set_hl("WarningMsg", HighlightOpt::with_fg(Yellow))?;
    set_hl("Question", HighlightOpt::with_fg(Green))?;

    set_hl("IncSearch", HighlightOpt::with_fg(Bg0).bg(Yellow))?;
    set_hl("Subsitube", HighlightOpt::with_fg(Bg0).bg(Green))?;

    set_hl("Yank", HighlightOpt::default().reverse())?;

    set_hl("Visual", HighlightOpt::with_fg(Bg0).bg(Purple))?;
    set_hl("EndOfBuffer", HighlightOpt::with_fg(Bg0))?;

    set_hl("LineNr", HighlightOpt::with_fg(Gray).bg(Bg1))?;
    set_hl("CursorLineNr", HighlightOpt::with_fg(Blue).bg(Bg0))?;

    set_hl("DiffAdd", HighlightOpt::with_fg(Green).bg(Bg0).bold().italic())?;
    set_hl("DiffChange", HighlightOpt::with_fg(Yellow).bg(Bg0).bold().italic())?;
    set_hl("DiffDelete", HighlightOpt::with_fg(Red).bg(Bg0).bold().italic(),)?;
    set_hl("DiffText", HighlightOpt::with_fg(Blue).bg(Bg0).bold().italic())?;

    set_hl("Changed", HighlightOpt::with_fg(Yellow).bold())?;
    set_hl("Added", HighlightOpt::with_fg(Green).bold())?;
    set_hl("Removed", HighlightOpt::with_fg(Red).bold())?;

    set_hl("WinSeparator", HighlightOpt::with_fg(Blue).bg(Bg1))?;

    set_hl("MatchParen", HighlightOpt::with_fg(Blue).bg(Bg1))?;

    set_hl("MsgArea", HighlightOpt::with_fg(Yellow).bg(Bg0))?;

    set_hl("SpellBad", HighlightOpt::with_fg(Red))?;
    set_hl("SpellCap", HighlightOpt::with_fg(Blue))?;
    set_hl("SpellLocal", HighlightOpt::with_fg(Yellow))?;
    set_hl("SpellRare", HighlightOpt::with_fg(Green))?;

    set_hl("DiagnosticError", HighlightOpt::with_fg(Red))?;
    set_hl("DiagnosticWarn", HighlightOpt::with_fg(Yellow))?;
    set_hl("DiagnosticInfo", HighlightOpt::with_fg(Blue))?;
    set_hl("DiagnosticHint", HighlightOpt::with_fg(Purple))?;
    set_hl("DiagnosticOk", HighlightOpt::with_fg(Green))?;

    set_hl("DiagnosticUnderlineError", HighlightOpt::with_fg(Red).underline())?;
    set_hl("DiagnosticUnderlineWarn", HighlightOpt::with_fg(Yellow).underline())?;
    set_hl("DiagnosticUnderlineInfo", HighlightOpt::with_fg(Blue).underline())?;
    set_hl("DiagnosticUnderlineHint", HighlightOpt::with_fg(Purple).underline())?;
    set_hl("DiagnosticUnderlineOk", HighlightOpt::with_fg(Green).underline())?;

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
    pub fn with_fg(color: Color) -> Self {
        Self::default().fg(color)
    }

    pub fn with_bg(color: Color) -> Self {
        Self::default().bg(color)
    }

    pub fn link(link: impl Into<&'static str>) -> Self {
        Self {
            link: Some(link.into()),
            ..Self::default()
        }
    }

    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }
    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }
    pub fn reverse(mut self) -> Self {
        self.reverse = true;
        self
    }
    pub fn strike(mut self) -> Self {
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
