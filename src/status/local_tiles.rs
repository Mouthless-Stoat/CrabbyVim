use mlua::{ObjectLike, Table};
use nvim_oxi::{Dictionary, api::Buffer};

use crate::{
    icons,
    options::get_option,
    plugins::devicons::get_icon,
    require, table,
    theme::{Color::*, HighlightOpt, get_hl, set_hl},
    vim, vim_fn,
};

use super::{STATUS_LINE_FG, Tile, TileStyle, eval_status};

pub struct GitDiff;

impl Tile for GitDiff {
    fn style(&self) -> TileStyle {
        TileStyle::Icon
    }

    fn icon(&self) -> nvim_oxi::Result<String> {
        Ok(icons::GIT_DIFF.into())
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

pub struct AltFileName(String);

impl AltFileName {
    pub fn new() -> Self {
        Self(String::new())
    }
}

impl Tile for AltFileName {
    fn content(&self) -> nvim_oxi::Result<String> {
        let file_name = self.0.clone();
        Ok(if file_name.is_empty() {
            String::new()
        } else {
            format!("alt: {file_name}")
        })
    }

    fn highlight_name(&self) -> nvim_oxi::Result<String> {
        Ok(format!("StatusAlt{}", get_icon(&self.0)?.1))
    }

    fn highlight_opt(&self) -> HighlightOpt {
        HighlightOpt::with_bg(Blue)
    }

    fn update(&mut self) -> nvim_oxi::Result<()> {
        self.0 = vim_fn("expand", "#:t")?;
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

pub struct Lsp(String);

impl Lsp {
    pub fn new() -> Self {
        Self(String::new())
    }
}

impl Tile for Lsp {
    fn style(&self) -> TileStyle {
        TileStyle::Icon
    }

    fn icon(&self) -> nvim_oxi::Result<String> {
        Ok(icons::LSP.into())
    }

    fn content(&self) -> nvim_oxi::Result<String> {
        Ok(vim()?
            .get::<Table>("lsp")?
            .call_function::<Table>(
                "get_clients",
                table! {
                    bufnr = Buffer::current().handle()
                },
            )?
            .get::<Table>(1)
            .map(|_| icons::GOOD)
            .unwrap_or(icons::BAD)
            .into())
    }

    fn highlight_name(&self) -> nvim_oxi::Result<String> {
        Ok(format!("StatusLsp{}", get_icon(&self.0)?.1))
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

pub struct Formatter(String);

impl Formatter {
    pub fn new() -> Self {
        Self(String::new())
    }
}

impl Tile for Formatter {
    fn style(&self) -> TileStyle {
        TileStyle::Icon
    }

    fn icon(&self) -> nvim_oxi::Result<String> {
        Ok(icons::FORMATTER.into())
    }

    fn content(&self) -> nvim_oxi::Result<String> {
        Ok(require("conform")?
            .call_function::<Table>("list_formatters", ())?
            .get::<Table>(1)
            .map(|_| icons::GOOD)
            .unwrap_or(icons::BAD)
            .into())
    }

    fn highlight_name(&self) -> nvim_oxi::Result<String> {
        Ok(format!("StatusFormatter{}", get_icon(&self.0)?.1))
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
