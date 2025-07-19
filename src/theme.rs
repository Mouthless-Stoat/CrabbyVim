mod syntax;

pub fn configure_highlight() -> nvim_oxi::Result<()> {
    nvim_highlight()?;
    syntax::syntax_highlight()?;
    Ok(())
}

#[rustfmt::skip]
fn nvim_highlight() -> nvim_oxi::Result<()> {
    set_hl( "Normal", HighlightOpt::new().fg(Color::White).bg(Color::Bg0))?;

    set_hl("NCursor", HighlightOpt::new().bg(Color::Blue))?;
    set_hl("ICursor", HighlightOpt::new().bg(Color::Green))?;
    set_hl("VCursor", HighlightOpt::new().bg(Color::Purple))?;
    set_hl("CCursor", HighlightOpt::new().bg(Color::Yellow))?;
    set_hl("RCursor", HighlightOpt::new().bg(Color::Red))?;

    set_hl("ErrorMsg", HighlightOpt::new().fg(Color::Red))?;
    set_hl("MoreMsg", HighlightOpt::new().fg(Color::Blue))?;
    set_hl("WarningMsg", HighlightOpt::new().fg(Color::Yellow))?;
    set_hl("Question", HighlightOpt::new().fg(Color::Green))?;

    set_hl("IncSearch", HighlightOpt::new().fg(Color::Bg0).bg(Color::Yellow))?;
    set_hl("Subsitube", HighlightOpt::new().fg(Color::Bg0).bg(Color::Green))?;

    set_hl("Yank", HighlightOpt::new().reverse())?;

    set_hl("Visual", HighlightOpt::new().fg(Color::Bg0).bg(Color::Purple))?;
    set_hl("EndOfBuffer", HighlightOpt::new().fg(Color::Bg0))?;

    set_hl("LineNr", HighlightOpt::new().fg(Color::Gray).bg(Color::Bg0))?;
    set_hl("CursorLineNr", HighlightOpt::new().fg(Color::Blue).bg(Color::Bg0))?;

    set_hl("DiffAdd", HighlightOpt::new().fg(Color::Green).bg(Color::Bg0).bold().italic())?;
    set_hl("DiffChange", HighlightOpt::new().fg(Color::Yellow).bg(Color::Bg0).bold().italic())?;
    set_hl("DiffDelete", HighlightOpt::new().fg(Color::Red).bg(Color::Bg0).bold().italic(),)?;
    set_hl("DiffText", HighlightOpt::new().fg(Color::Blue).bg(Color::Bg0).bold().italic())?;

    set_hl("Changed", HighlightOpt::new().fg(Color::Yellow).bold())?;
    set_hl("Added", HighlightOpt::new().fg(Color::Green).bold())?;
    set_hl("Removed", HighlightOpt::new().fg(Color::Red).bold())?;

    set_hl("WinSeparator", HighlightOpt::new().fg(Color::Blue).bg(Color::Bg1))?;

    set_hl("MatchParen", HighlightOpt::new().fg(Color::Blue).bg(Color::Bg1))?;

    set_hl("MsgArea", HighlightOpt::new().fg(Color::Yellow).bg(Color::Bg0))?;

    set_hl("SpellBad", HighlightOpt::new().fg(Color::Red))?;
    set_hl("SpellCap", HighlightOpt::new().fg(Color::Blue))?;
    set_hl("SpellLocal", HighlightOpt::new().fg(Color::Yellow))?;
    set_hl("SpellRare", HighlightOpt::new().fg(Color::Green))?;

    set_hl("DiagnosticError", HighlightOpt::new().fg(Color::Red))?;
    set_hl("DiagnosticWarn", HighlightOpt::new().fg(Color::Yellow))?;
    set_hl("DiagnosticInfo", HighlightOpt::new().fg(Color::Blue))?;
    set_hl("DiagnosticHint", HighlightOpt::new().fg(Color::Purple))?;
    set_hl("DiagnosticOk", HighlightOpt::new().fg(Color::Green))?;

    set_hl("DiagnosticUnderlineError", HighlightOpt::new().fg(Color::Red).underline())?;
    set_hl("DiagnosticUnderlineWarn", HighlightOpt::new().fg(Color::Yellow).underline())?;
    set_hl("DiagnosticUnderlineInfo", HighlightOpt::new().fg(Color::Blue).underline())?;
    set_hl("DiagnosticUnderlineHint", HighlightOpt::new().fg(Color::Purple).underline())?;
    set_hl("DiagnosticUnderlineOk", HighlightOpt::new().fg(Color::Green).underline())?;

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
            fn as_str(self) -> &'static str {
                match self {
                    $(Self::$name => $value,)*
                }
            }
        }
    }
}

colors! {
    Pink = "#ff0084";
    Red = "#f65866";
    Orange = "#fa9534";
    Yellow = "#efbd5d";
    Green = "#8bcd5b";
    Cyan = "#34bfd0";
    Blue = "#41a7fc";
    Purple = "#c75ae8";

    White = "#93a4c3";
    Gray = "#455574";
    Bg0 = "#020a20"; // darkest
    Bg1 = "#09152c";
    Bg2 = "#142139";
    Bg3 = "#202e47"; // lightest

}

// Not using SetHighlightOpts by nvim_oxi because it is too complex with too many feature that we
// never use
#[derive(Clone, Copy)]
pub struct HighlightOpt {
    fg: Option<Color>,
    bg: Option<Color>,
    underline: bool,
    bold: bool,
    italic: bool,
    reverse: bool,
    link: Option<&'static str>,
}

impl HighlightOpt {
    fn new() -> Self {
        HighlightOpt {
            fg: None,
            bg: None,
            underline: false,
            bold: false,
            italic: false,
            reverse: false,
            link: None,
        }
    }

    fn link(hl: impl Into<&'static str>) -> Self {
        let mut t = Self::new();
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
}

pub fn set_hl<'a>(name: impl Into<&'a str>, opt: HighlightOpt) -> nvim_oxi::Result<()> {
    let mut opt_builder = nvim_oxi::api::opts::SetHighlightOpts::builder();

    if let Some(link) = opt.link {
        opt_builder.link(link);
    } else {
        if let Some(fg) = opt.fg {
            opt_builder.foreground(fg.as_str());
        }
        if let Some(bg) = opt.bg {
            opt_builder.background(bg.as_str());
        }

        opt_builder.underline(opt.underline);
        opt_builder.bold(opt.bold);
        opt_builder.italic(opt.italic);
    }
    opt_builder.force(true);

    nvim_oxi::api::set_hl(0, name.into(), &opt_builder.build())?;

    Ok(())
}
