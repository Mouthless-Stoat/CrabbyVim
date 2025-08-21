// If you want a lua version of this to include in your config check out
// https://github.com/Mouthless-Stoat/Nvim-config/blob/913333d01835ac974d7079bdc5cf9fbb03d869a5/lua/config/theme/plugins/status.lua

use mlua::Table;

use crate::theme::Color::*;
use crate::theme::HighlightOpt;
use crate::theme::set_hl;
use crate::vim;

use super::STATUS_LINE_FG;
use super::{Tile, TileStyle};

pub struct Mode(crate::Mode);

impl Mode {
    pub fn new() -> Self {
        Mode(crate::Mode::Normal)
    }
}

impl Tile for Mode {
    fn content(&self) -> nvim_oxi::Result<String> {
        Ok(self.0.as_str().into())
    }

    fn highlight_name(&self) -> nvim_oxi::Result<&'static str> {
        Ok("StatusMode")
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

pub struct Git;

impl Tile for Git {
    fn style(&self) -> TileStyle {
        TileStyle::Icon
    }

    fn icon(&self) -> nvim_oxi::Result<String> {
        Ok("".into())
    }

    fn content(&self) -> nvim_oxi::Result<String> {
        let head = match vim()?.get::<Table>("g")?.get::<String>("gitsigns_head") {
            Ok(head) if head.is_empty() => return Ok(String::new()),
            Ok(head) => head,
            Err(mlua::Error::FromLuaConversionError { from: "nil", .. }) => {
                return Ok(String::new());
            }
            Err(err) => return Err(nvim_oxi::Error::Mlua(err)),
        };

        let (added, changed, removed) = match vim()?
            .get::<Table>("b")?
            .get::<Table>("gitsigns_status_dict")
        {
            Ok(dict) => (
                dict.get::<usize>("added").unwrap_or(0),
                dict.get::<usize>("changed").unwrap_or(0),
                dict.get::<usize>("removed").unwrap_or(0),
            ),
            Err(mlua::Error::FromLuaConversionError { from: "nil", .. }) => {
                return Ok(head);
            }
            Err(err) => return Err(nvim_oxi::Error::Mlua(err)),
        };

        let mut out = vec![];

        if added > 0 {
            out.push(format!("%#StatusGitAdd#+{added}"));
        }
        if changed > 0 {
            out.push(format!("%#StatusGitChange#+{changed}"));
        }
        if removed > 0 {
            out.push(format!("%#StatusGitRemove#+{removed}"));
        }

        let mut diff = out.join(" ");
        if !diff.is_empty() {
            diff.insert(0, ' ');
        }

        Ok(format!("{head}{diff}"))
    }

    fn highlight_name(&self) -> nvim_oxi::Result<&'static str> {
        Ok("StatusGit")
    }

    fn highlight_opt(&self) -> HighlightOpt {
        HighlightOpt::with_bg(Orange)
    }

    fn setup(&self) -> nvim_oxi::Result<()> {
        set_hl(
            "StatusGitAdd",
            HighlightOpt::with_fg(Green).bg(STATUS_LINE_FG),
        )?;
        set_hl(
            "StatusGitChange",
            HighlightOpt::with_fg(Yellow).bg(STATUS_LINE_FG),
        )?;
        set_hl(
            "StatusGitRemove",
            HighlightOpt::with_fg(Red).bg(STATUS_LINE_FG),
        )?;
        Ok(())
    }
}

pub struct Loc;

impl Tile for Loc {
    fn content(&self) -> nvim_oxi::Result<String> {
        Ok("%3.c:%-3.l".into())
    }

    fn highlight_name(&self) -> nvim_oxi::Result<&'static str> {
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
        Ok("󰍉".into())
    }

    fn content(&self) -> nvim_oxi::Result<String> {
        Ok(format!(
            "{:.0}%%",
            nvim_oxi::api::get_var::<f64>("neovide_scale_factor")? * 100.0
        ))
    }

    fn highlight_name(&self) -> nvim_oxi::Result<&'static str> {
        Ok("StatusZoom")
    }

    fn highlight_opt(&self) -> HighlightOpt {
        HighlightOpt::with_bg(Yellow)
    }
}
