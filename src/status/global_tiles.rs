use mlua::{ObjectLike, Table};
use nvim_oxi::api::get_var;

use crate::{
    diagnostic::DiagnosticSeverity,
    icons, table,
    theme::{
        Color::{self, *},
        HighlightOpt, set_hl,
    },
    vim,
};

use super::{STATUS_LINE_FG, Tile, TileStyle};

pub struct Mode(crate::Mode);

impl Mode {
    #[must_use]
    pub fn new() -> Self {
        Mode(crate::Mode::Normal)
    }
}

impl Default for Mode {
    fn default() -> Self {
        Self::new()
    }
}

impl Tile for Mode {
    fn content(&self) -> nvim_oxi::Result<String> {
        Ok(self.0.as_str().into())
    }

    fn highlight_name(&self) -> nvim_oxi::Result<String> {
        Ok("StatusMode".into())
    }

    fn highlight_opt(&self) -> HighlightOpt {
        HighlightOpt::with_bg(Blue)
    }

    fn update_highlight(&self, old_opt: HighlightOpt) -> nvim_oxi::Result<HighlightOpt> {
        Ok(old_opt.bg(match self.0 {
            crate::Mode::Normal => Blue,
            crate::Mode::Insert => Green,
            crate::Mode::Command => Yellow,
            crate::Mode::Visual => Purple,
            crate::Mode::Replace => Red,
            crate::Mode::Terminal => Cyan,
        }))
    }

    fn update(&mut self) -> nvim_oxi::Result<()> {
        self.0 = crate::Mode::current_mode()?;
        Ok(())
    }
}

pub struct Cwd(String);

impl Cwd {
    #[must_use]
    pub fn new() -> Self {
        Cwd(String::new())
    }

    fn map_path(&self) -> (&'static str, String, Color) {
        match self.0.as_str() {
            r"D:\OneDrive\Desktop\Code" => (icons::CODE_CWD, "code".into(), Blue),
            r"D:\OneDrive\Desktop" => (icons::DESKTOP_CWD, "desktop".into(), Orange),
            r"D:\config\nvim" => (icons::NVIM_CWD, "nvim".into(), Green),
            r"C:\Users\nphuy" => (icons::HOME_CWD, "home".into(), Yellow),
            p => (icons::FOLDER, p.into(), Yellow),
        }
    }
}

impl Default for Cwd {
    fn default() -> Self {
        Self::new()
    }
}

impl Tile for Cwd {
    fn style(&self) -> TileStyle {
        TileStyle::Icon
    }

    fn icon(&self) -> nvim_oxi::Result<String> {
        Ok(self.map_path().0.into())
    }

    fn content(&self) -> nvim_oxi::Result<String> {
        Ok(self.map_path().1)
    }

    fn highlight_name(&self) -> nvim_oxi::Result<String> {
        Ok("StatusCwd".into())
    }

    fn highlight_opt(&self) -> HighlightOpt {
        HighlightOpt::with_bg(Yellow)
    }

    fn update_highlight(&self, old_opt: HighlightOpt) -> nvim_oxi::Result<HighlightOpt> {
        Ok(old_opt.bg(self.map_path().2))
    }

    fn update(&mut self) -> nvim_oxi::Result<()> {
        self.0 = nvim_oxi::api::call_function("getcwd", ((),))?;
        Ok(())
    }
}

pub struct GitBranch;
impl Tile for GitBranch {
    fn style(&self) -> TileStyle {
        TileStyle::Icon
    }

    fn icon(&self) -> nvim_oxi::Result<String> {
        Ok(icons::GIT_BRANCH.into())
    }

    fn content(&self) -> nvim_oxi::Result<String> {
        match get_var::<String>("gitsigns_head") {
            Ok(head) => Ok(head),
            _ => Ok(String::new()),
        }
    }

    fn highlight_name(&self) -> nvim_oxi::Result<String> {
        Ok("StatusGit".into())
    }

    fn highlight_opt(&self) -> HighlightOpt {
        HighlightOpt::with_bg(Orange)
    }
}

pub struct Loc;

impl Tile for Loc {
    fn content(&self) -> nvim_oxi::Result<String> {
        Ok("%3.c:%-3.l".into())
    }

    fn highlight_name(&self) -> nvim_oxi::Result<String> {
        Mode(crate::Mode::Normal).highlight_name()
    }

    fn highlight_opt(&self) -> HighlightOpt {
        Mode(crate::Mode::Normal).highlight_opt()
    }
}

pub struct Zoom;

impl Tile for Zoom {
    fn style(&self) -> TileStyle {
        TileStyle::Icon
    }

    fn icon(&self) -> nvim_oxi::Result<String> {
        Ok(icons::MAGNIFYING_GLASS.into())
    }

    fn content(&self) -> nvim_oxi::Result<String> {
        Ok(format!(
            "{:.0}%%",
            nvim_oxi::api::get_var::<f64>("neovide_scale_factor")? * 100.0
        ))
    }

    fn highlight_name(&self) -> nvim_oxi::Result<String> {
        Ok("StatusZoom".into())
    }

    fn highlight_opt(&self) -> HighlightOpt {
        HighlightOpt::with_bg(Yellow)
    }
}

#[derive(Default)]
struct DiagnosticCount {
    error: i64,
    warn: i64,
    hint: i64,
    info: i64,
}

impl DiagnosticCount {
    /// Calculate the total amount of diagnostics excluding the info and hint diagnostics.
    fn total(&self) -> i64 {
        self.error + self.warn
    }
}

pub struct Diagnostic(bool, DiagnosticCount);

impl Diagnostic {
    #[must_use]
    pub fn new(is_global: bool) -> Self {
        Self(is_global, DiagnosticCount::default())
    }
}

impl Tile for Diagnostic {
    fn style(&self) -> TileStyle {
        if self.0 {
            TileStyle::Icon
        } else {
            TileStyle::Bubble
        }
    }

    fn icon(&self) -> nvim_oxi::Result<String> {
        Ok(icons::FOLDER.into())
    }

    fn content(&self) -> nvim_oxi::Result<String> {
        let mut out = vec![];

        let DiagnosticCount {
            error,
            warn,
            hint,
            info,
        } = self.1;

        if error > 0 {
            out.push(format!("%#StatusError#{} {error}", icons::ERROR));
        }
        if warn > 0 {
            out.push(format!("%#StatusWarn#{} {warn}", icons::WARN));
        }
        if hint > 0 {
            out.push(format!("%#StatusHint#{} {hint}", icons::HINT));
        }
        if info > 0 {
            out.push(format!("%#StatusInfo#{} {info}", icons::INFO));
        }

        Ok(out.join(" "))
    }

    fn highlight_name(&self) -> nvim_oxi::Result<String> {
        Ok(if self.0 {
            "StatusDiagnosticGlobal"
        } else {
            "StatusDiagnostic"
        }
        .into())
    }

    fn highlight_opt(&self) -> HighlightOpt {
        HighlightOpt::with_bg(Red)
    }

    fn update_highlight(&self, _old_opt: HighlightOpt) -> nvim_oxi::Result<HighlightOpt> {
        Ok(if self.0 {
            let total = self.1.total();
            let color = match total {
                0 => Purple,
                1..5 => Yellow,
                5..10 => Orange,
                _ => Red,
            };
            HighlightOpt::with_bg(color)
        } else {
            HighlightOpt::with_bg(STATUS_LINE_FG)
        })
    }

    fn setup(&self) -> nvim_oxi::Result<()> {
        set_hl("StatusError", HighlightOpt::with_fg(Red).bg(STATUS_LINE_FG))?;
        set_hl(
            "StatusWarn",
            HighlightOpt::with_fg(Yellow).bg(STATUS_LINE_FG),
        )?;
        set_hl("StatusInfo", HighlightOpt::with_fg(Blue).bg(STATUS_LINE_FG))?;
        set_hl(
            "StatusHint",
            HighlightOpt::with_fg(Purple).bg(STATUS_LINE_FG),
        )?;
        Ok(())
    }

    fn update(&mut self) -> nvim_oxi::Result<()> {
        let get_diagnostic = |severity: DiagnosticSeverity| -> nvim_oxi::Result<i64> {
            Ok(vim()?
                .get::<Table>("diagnostic")?
                .call_function::<Table>(
                    "get",
                    (
                        if self.0 {
                            mlua::Nil
                        } else {
                            mlua::Value::Integer(0)
                        },
                        table! {severity = severity},
                    ),
                )?
                .len()?)
        };

        self.1 = DiagnosticCount {
            error: get_diagnostic(DiagnosticSeverity::Error)?,
            warn: get_diagnostic(DiagnosticSeverity::Warn)?,
            hint: get_diagnostic(DiagnosticSeverity::Hint)?,
            info: get_diagnostic(DiagnosticSeverity::Info)?,
        };

        Ok(())
    }
}
