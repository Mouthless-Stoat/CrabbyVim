// If you want a lua version of this to include in your config check out
// https://github.com/Mouthless-Stoat/Nvim-config/blob/913333d01835ac974d7079bdc5cf9fbb03d869a5/lua/config/theme/plugins/status.lua

use nvim_oxi::Dictionary;
use nvim_oxi::api::Buffer;
use nvim_oxi::api::get_var;

use crate::icons;
use crate::options::get_option;
use crate::plugins::devicons::get_icon;
use crate::theme::Color;
use crate::theme::Color::*;
use crate::theme::HighlightOpt;
use crate::theme::get_hl;
use crate::theme::set_hl;

use super::STATUS_LINE_FG;
use super::eval_status;
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

pub struct Git;
impl Tile for Git {
    fn style(&self) -> TileStyle {
        TileStyle::Icon
    }

    fn icon(&self) -> nvim_oxi::Result<String> {
        Ok("".into())
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

pub struct GitDiff;

impl Tile for GitDiff {
    fn style(&self) -> TileStyle {
        TileStyle::Icon
    }

    fn icon(&self) -> nvim_oxi::Result<String> {
        Ok("".into())
    }

    fn content(&self) -> nvim_oxi::Result<String> {
        let (added, changed, removed) =
            match Buffer::current().get_var::<Dictionary>("gitsigns_status_dict") {
                Ok(dict) => (
                    dict.get("added")
                        .map_or(0, |o| unsafe { o.as_integer_unchecked() }),
                    dict.get("changed")
                        .map_or(0, |o| unsafe { o.as_integer_unchecked() }),
                    dict.get("removed")
                        .map_or(0, |o| unsafe { o.as_integer_unchecked() }),
                ),
                Err(_) => (0, 0, 0),
            };

        let mut out = vec![];

        if added > 0 {
            out.push(format!("%#StatusGitAdd#+{added}"));
        }
        if changed > 0 {
            out.push(format!("%#StatusGitChange#~{changed}"));
        }
        if removed > 0 {
            out.push(format!("%#StatusGitRemove#-{removed}"));
        }

        Ok(out.join(" "))
    }

    fn highlight_name(&self) -> nvim_oxi::Result<String> {
        Ok("StatusGitDiff".into())
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
        Ok("󰍉".into())
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

pub struct FileName(String);

impl FileName {
    pub fn new() -> Self {
        Self(String::new())
    }
}

impl Tile for FileName {
    fn style(&self) -> TileStyle {
        TileStyle::Icon
    }

    fn icon(&self) -> nvim_oxi::Result<String> {
        Ok(get_icon(&self.0)?.0)
    }

    fn content(&self) -> nvim_oxi::Result<String> {
        Ok(self.0.clone())
    }

    fn highlight_name(&self) -> nvim_oxi::Result<String> {
        Ok(format!("Status{}", get_icon(&self.0)?.1))
    }

    fn highlight_opt(&self) -> HighlightOpt {
        HighlightOpt::with_bg(Blue)
    }

    fn update(&mut self) -> nvim_oxi::Result<()> {
        self.0 = eval_status("%t")?.str;
        Ok(())
    }

    fn update_highlight(&self, _old_opt: HighlightOpt) -> nvim_oxi::Result<HighlightOpt> {
        Ok(get_hl(get_icon(&self.0)?.1)?.reverse_fg_bg())
    }
}

enum FileStatusFlag {
    Modified,
    UnModifiable,
    None,
}

pub struct FileStatus(FileStatusFlag);

impl FileStatus {
    pub fn new() -> Self {
        FileStatus(FileStatusFlag::None)
    }
}

impl Tile for FileStatus {
    fn content(&self) -> nvim_oxi::Result<String> {
        Ok(match self.0 {
            FileStatusFlag::Modified => "[+]",
            FileStatusFlag::UnModifiable => "[-]",
            FileStatusFlag::None => "",
        }
        .into())
    }

    fn highlight_name(&self) -> nvim_oxi::Result<String> {
        Ok(match self.0 {
            FileStatusFlag::Modified => "StatusFileMod",
            FileStatusFlag::UnModifiable => "StatusFileUnMod",
            FileStatusFlag::None => "",
        }
        .into())
    }

    fn highlight_opt(&self) -> HighlightOpt {
        HighlightOpt::default()
    }

    fn update_highlight(&self, old_opt: HighlightOpt) -> nvim_oxi::Result<HighlightOpt> {
        Ok(match self.0 {
            FileStatusFlag::Modified => HighlightOpt::with_bg(Green),
            FileStatusFlag::UnModifiable => HighlightOpt::with_bg(Red),
            FileStatusFlag::None => old_opt,
        })
    }

    fn update(&mut self) -> nvim_oxi::Result<()> {
        self.0 = if !get_option("modifiable")? {
            FileStatusFlag::UnModifiable
        } else if get_option("modified")? {
            FileStatusFlag::Modified
        } else {
            FileStatusFlag::None
        };

        Ok(())
    }
}
