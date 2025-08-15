use crate::options::set_option;
use nvim_oxi::api::set_var;

mod syntax;

pub fn configure() -> nvim_oxi::Result<()> {
    highlights()?;
    syntax::highlights()?;

    Ok(())
}

pub fn configure_highlights(hls: Vec<(&'static str, HighlightOpt)>) -> nvim_oxi::Result<()> {
    for hl in hls {
        set_hl(hl.0, hl.1)?;
    }
    Ok(())
}

#[rustfmt::skip]
fn highlights() -> nvim_oxi::Result<()> {
    use Color::*;

    set_option("guicursor", "n-o:block-NCursor,i:ver20-ICursor,v-ve:block-VCursor,c-ci-cr:ver25-CCursor,r:hor15-RCursor")?;

    set_var("terminal_color_0", Bg0.to_str())?;
    set_var("terminal_color_1", Red.to_str())?;
    set_var("terminal_color_2", Green.to_str())?;
    set_var("terminal_color_3", Yellow.to_str())?;
    set_var("terminal_color_4", Blue.to_str())?;
    set_var("terminal_color_5", Purple.to_str())?;
    set_var("terminal_color_6", Cyan.to_str())?;
    set_var("terminal_color_7", White.to_str())?;
    set_var("terminal_color_8", Gray.to_str())?;
    set_var("terminal_color_9", Red.to_str())?;
    set_var("terminal_color_10", Green.to_str())?;
    set_var("terminal_color_11", Yellow.to_str())?;
    set_var("terminal_color_12", Blue.to_str())?;
    set_var("terminal_color_13", Purple.to_str())?;
    set_var("terminal_color_14", Cyan.to_str())?;
    set_var("terminal_color_15", White.to_str())?;

    configure_highlights(vec![
        ("Normal",HighlightOpt::with_fg(White).bg(Bg0)),
        ("NormalFloat",HighlightOpt::with_fg(White).bg(Bg0)),
        ("FloatBorder", HighlightOpt::with_fg(Gray)),

        ("MoreMsg", HighlightOpt::with_fg(Green)),
        ("QuickFixLine", HighlightOpt::with_fg(Blue)),

        ("Title", HighlightOpt::with_fg(Blue)),

        ("Directory", HighlightOpt::with_fg(Blue)),

        ("NCursor",HighlightOpt::with_bg(Blue)),
        ("ICursor",HighlightOpt::with_bg(Green)),
        ("VCursor",HighlightOpt::with_bg(Purple)),
        ("CCursor",HighlightOpt::with_bg(Yellow)),
        ("RCursor",HighlightOpt::with_bg(Red)),

        ("ErrorMsg",HighlightOpt::with_fg(Red)),
        ("MoreMsg",HighlightOpt::with_fg(Blue)),
        ("WarningMsg",HighlightOpt::with_fg(Yellow)),
        ("Question",HighlightOpt::with_fg(Green)),

        ("IncSearch",HighlightOpt::with_fg(Bg0).bg(Yellow)),
        ("Substitute",HighlightOpt::with_fg(Bg0).bg(Purple)),

        ("Yank",HighlightOpt::default().reverse()),

        ("Visual",HighlightOpt::with_bg(Bg2)),
        ("EndOfBuffer",HighlightOpt::with_fg(Bg0)),

        ("LineNr",HighlightOpt::with_fg(Gray).bg(Bg1)),
        ("CursorLineNr",HighlightOpt::with_fg(Blue).bg(Bg0)),

        ("DiffAdd",HighlightOpt::with_fg(Green).bg(Bg0).bold().italic()),
        ("DiffChange",HighlightOpt::with_fg(Yellow).bg(Bg0).bold().italic()),
        ("DiffDelete",HighlightOpt::with_fg(Red).bg(Bg0).bold().italic(),),
        ("DiffText",HighlightOpt::with_fg(Blue).bg(Bg0).bold().italic()),

        ("Changed",HighlightOpt::with_fg(Yellow).bold()),
        ("Added",HighlightOpt::with_fg(Green).bold()),
        ("Removed",HighlightOpt::with_fg(Red).bold()),

        ("WinSeparator",HighlightOpt::with_fg(Blue).bg(Bg1)),

        ("MatchParen",HighlightOpt::link("Visual")),

        ("MsgArea",HighlightOpt::with_fg(Yellow).bg(Bg0)),

        ("SpellBad",HighlightOpt::with_fg(Red)),
        ("SpellCap",HighlightOpt::with_fg(Blue)),
        ("SpellLocal",HighlightOpt::with_fg(Yellow)),
        ("SpellRare",HighlightOpt::with_fg(Green)),

        ("DiagnosticError",HighlightOpt::with_fg(Red)),
        ("DiagnosticWarn",HighlightOpt::with_fg(Yellow)),
        ("DiagnosticInfo",HighlightOpt::with_fg(Blue)),
        ("DiagnosticHint",HighlightOpt::with_fg(Purple)),
        ("DiagnosticOk",HighlightOpt::with_fg(Green)),

        ("DiagnosticUnderlineError",HighlightOpt::with_fg(Red).underline()),
        ("DiagnosticUnderlineWarn",HighlightOpt::with_fg(Yellow).underline()),
        ("DiagnosticUnderlineInfo",HighlightOpt::with_fg(Blue).underline()),
        ("DiagnosticUnderlineHint",HighlightOpt::with_fg(Purple).underline()),
        ("DiagnosticUnderlineOk",HighlightOpt::with_fg(Green).underline()),
    ])?;
        
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
            pub fn to_str(self) -> &'static str {
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

    Bg0 = "#101010"; // default bg
    Bg1 = "#242424";
    Bg2 = "#404040";
    Bg3 = "#5e5e5e"; // lightest
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

pub fn set_hl<'a>(name: impl Into<&'a str>, opt: impl Into<HighlightOpt>) -> nvim_oxi::Result<()> {
    let opt = opt.into();

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
