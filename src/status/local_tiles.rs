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

/// Tile to show the git diff of the current file with the help of
/// [`gitsigns.nvim`](https://github.com/lewis6991/gitsigns.nvim)
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

/// Tile to show the current file name.
///
/// The tile also change the icon/color to match the file type using `nvim-web-devicons`
pub struct FileName(String);

impl FileName {
    #[must_use]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self(String::new())
    }
}

impl Default for FileName {
    fn default() -> Self {
        Self::new()
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

/// Tile to show the alternate file name.
///
/// The tile also change the color to match the file type using `nvim-web-devicons`.
pub struct AltFileName(String);

impl AltFileName {
    #[must_use]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self(String::new())
    }
}

impl Default for AltFileName {
    fn default() -> Self {
        Self::new()
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

/// Tile to show the current status of the current file.
///
/// Status are if the file was modified or can it even be modify. Use a `[+]` for if the file was
/// modified and a `[-]` if the file can't be modify.
pub struct FileStatus(FileStatusFlag);

impl FileStatus {
    #[must_use]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        FileStatus(FileStatusFlag::None)
    }
}

impl Default for FileStatus {
    fn default() -> Self {
        Self::new()
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

/// Tile to show if a LSP is attach to the current file.
///
/// This tile simply show a tick mark if there is a lsp and a cross if a lsp is not found. The
/// color of this file also change to match the current file type using `nvim-web-devicons`.
pub struct Lsp(String);

impl Lsp {
    #[must_use]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self(String::new())
    }
}

impl Default for Lsp {
    fn default() -> Self {
        Self::new()
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

/// Tile to show if a formatter is attach to the current file using
/// [`conform.nvim`](https://github.com/stevearc/conform.nvim)
///
/// This tile simply show a tick mark if there is a formatter and a cross if a formatter is not found. The
/// color of this file also change to match the current file type using `nvim-web-devicons`.
pub struct Formatter(String);

impl Formatter {
    #[must_use]
    #[allow(missing_docs)]
    pub fn new() -> Formatter {
        Self(String::new())
    }
}

impl Default for Formatter {
    fn default() -> Self {
        Self::new()
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

/// This tile show the status of tools attached to the current file.
///
/// The tile will display a gear icon if only a lsp is found, a paint brush if only a formatter is
/// found, if both are found a tick mark will be display and if neither are found a cross is shown.
/// The Tile also change color based on the file type using `nvim-web-devicons`.
pub struct Tools(String);

impl Tools {
    #[must_use]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self(String::new())
    }
}

impl Default for Tools {
    fn default() -> Self {
        Self::new()
    }
}

impl Tile for Tools {
    fn content(&self) -> nvim_oxi::Result<String> {
        let formatter_attach = require("conform")?
            .call_function::<Table>("list_formatters", ())?
            .len()?
            > 0;

        let lsp_attach = vim()?
            .get::<Table>("lsp")?
            .call_function::<Table>(
                "get_clients",
                table! {
                    bufnr = Buffer::current().handle()
                },
            )?
            .len()?
            > 0;

        Ok(match (formatter_attach, lsp_attach) {
            (true, true) => icons::GOOD,
            (true, false) => icons::FORMATTER,
            (false, true) => icons::LSP,
            (false, false) => icons::BAD,
        }
        .into())
    }

    fn highlight_name(&self) -> nvim_oxi::Result<String> {
        Ok(format!("StatusTools{}", get_icon(&self.0)?.1))
    }

    fn highlight_opt(&self) -> HighlightOpt {
        HighlightOpt::with_bg(Blue)
    }

    fn update(&mut self) -> nvim_oxi::Result<()> {
        self.0 = eval_status("%t")?.str;
        Ok(())
    }

    fn update_highlight(&self, _old_opt: HighlightOpt) -> nvim_oxi::Result<HighlightOpt> {
        Ok(get_hl(get_icon(&self.0)?.1)?.bg(STATUS_LINE_FG))
    }
}
