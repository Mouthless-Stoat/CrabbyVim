use mlua::Table;

use crate::theme::Color::*;
use crate::theme::HighlightOpt;
use crate::vim;

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
        match vim()?.get::<Table>("g")?.get::<String>("gitsigns_head") {
            Ok(head) if head.is_empty() => Ok("n/a".into()),
            Ok(head) => Ok(head),
            Err(err) => match err {
                mlua::Error::FromLuaConversionError { from: "nil", .. } => Ok(String::new()),
                _ => todo!(),
            },
        }
    }

    fn highlight_name(&self) -> nvim_oxi::Result<&'static str> {
        Ok("StatusGit")
    }

    fn highlight_opt(&self) -> HighlightOpt {
        HighlightOpt::with_bg(Orange)
    }
}

pub struct GitChange;

impl Tile for GitChange {
    fn content(&self) -> nvim_oxi::Result<String> {
        todo!()
    }

    fn highlight_name(&self) -> nvim_oxi::Result<&'static str> {
        todo!()
    }

    fn highlight_opt(&self) -> HighlightOpt {
        todo!()
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
