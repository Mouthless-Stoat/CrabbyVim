//! A Custom bubble like statusline information that can also be use for `winbar` and `tabline`.
//!
//! You should set the following highlight group to get the best result out of this module
//!```rust
//! configure_highlights(vec![
//!     ("StatusLine", HighlightOpt::with_bg(STATUS_LINE_BG).fg(STATUS_LINE_FG)),
//!     ("StatusLineNC", HighlightOpt::with_bg(STATUS_LINE_BG).fg(STATUS_LINE_FG)),
//!     ("WinBar", HighlightOpt::with_bg(STATUS_LINE_BG).fg(STATUS_LINE_FG)),
//!     ("WinBarNc", HighlightOpt::with_bg(STATUS_LINE_BG).fg(STATUS_LINE_FG)),
//! ])?;
//!```
//!
//! To create a new statusline (or `winbar` and `tabline`) start with `Line::new()` to make struct
//! to build up the line components. After the line is finish make a lua function that run
//! `Line::render()` to render the line and lastly attach this lua function to the `statusline`
//! neovim option. You might want to also create a few autocmds to redraw the statusline.
//!
//! A bunch of existing common components are provided but more can be created by makign a new type
//! that implemented the [`Tile`] trait. A few tile require the use of a `new()` method like
//! ([`Mode`], [`FileName`], etc.) because they use a cache to help with rendering the tile faster
//! without refetching the information multiple time.

// If you want a lua version of this statusline check out the old version:
// https://github.com/Mouthless-Stoat/Nvim-config/blob/913333d01835ac974d7079bdc5cf9fbb03d869a5/lua/config/theme/plugins/status.lua

use nvim_oxi::api::types::StatuslineInfos;

use crate::{
    autocmds::create_autocmd_cmd,
    options::{get_option, set_option},
    theme::{Color, HighlightOpt, configure_highlights, set_hl},
};

mod global_tiles;
pub use global_tiles::*;

mod local_tiles;
pub use local_tiles::*;

/// The color of the statusline background
pub const STATUS_LINE_BG: Color = crate::theme::Color::Bg1;
/// The color of the statusline foreground
pub const STATUS_LINE_FG: Color = crate::theme::Color::Bg2;

pub(crate) fn configure() -> nvim_oxi::Result<()> {
    #[rustfmt::skip]
    configure_highlights(vec![
        ("StatusLine", HighlightOpt::with_bg(STATUS_LINE_BG).fg(STATUS_LINE_FG)),
        ("StatusLineNC", HighlightOpt::with_bg(STATUS_LINE_BG).fg(STATUS_LINE_FG)),
        ("WinBar", HighlightOpt::with_bg(STATUS_LINE_BG).fg(STATUS_LINE_FG)),
        ("WinBarNc", HighlightOpt::with_bg(STATUS_LINE_BG).fg(STATUS_LINE_FG)),
    ])?;

    set_option("laststatus", 3)?;

    let mut statusline = Line::new();

    statusline.add_left(Mode::new());
    statusline.add_left(Cwd::new());
    statusline.add_left(GitBranch);
    statusline.add_left(Diagnostic::new(true));
    if nvim_oxi::api::get_var::<bool>("neovide").is_ok() {
        statusline.add_right(Zoom);
    }
    statusline.add_right(Loc);

    let mut winbar = Line::new();

    winbar.add_left(GitDiff);
    winbar.add_left(Diagnostic::new(false));
    winbar.add_center(FileName::new());
    winbar.add_right_center(Tools::new());
    winbar.add_right_center(FileStatus::new());
    winbar.add_right(AltFileName::new());

    #[rustfmt::skip]
    nvim_oxi::mlua::lua().globals().set(
        "statusline",
        nvim_oxi::mlua::lua().create_function_mut(move |_, ()| {
            Ok(statusline.render().expect("Can't render statusline"))
        })?,
    )?;

    #[rustfmt::skip]
    nvim_oxi::mlua::lua().globals().set(
        "winbar",
        nvim_oxi::mlua::lua() .create_function_mut(move |_, ()| {
            Ok(winbar.render().expect("Can't render winbar"))
        })?,
    )?;

    set_option("statusline", "%!v:lua.statusline()")?;
    set_option("winbar", "%{%v:lua.winbar()%}")?;

    create_autocmd_cmd(&["User"], &["GitSignsUpdate"], "redrawstatus!")?;
    create_autocmd_cmd(&["DiagnosticChanged"], &["*"], "redrawstatus!")?;

    Ok(())
}

/// Style for a given statusline tile.
pub enum TileStyle {
    /// Content of the tile wrap in a little bubble all with the same background color
    Bubble,
    /// Content of the tile next to an icon with background color while the main contain only have
    /// it foreground be colored.
    Icon,
}

/// Trait for a tile to implement. Each tile need to implement:
/// [`Tile::content`], [`Tile::highlight_name`] and [`Tile::highlight_opt`].
/// - [`Tile::content`] return what need to be render within each tile
/// - [`Tile::highlight_name`] return the highlight group that will be use to highlight the tile.
///   This should only color the the background of the highlight group, the foreground and everything
///   else will be calculate by the render.
/// - [`Tile::highlight_opt`] return the option to setup the highlight group for the first time.
///
/// Other important method like [`Tile::setup`] and [`Tile::update`] use to create or update the
/// value.
pub trait Tile {
    /// The style of this tile.
    ///
    /// If this option return [`TileStyle::Icon`], [`Tile::icon`] must be
    /// implemented as well to have an icon to render.
    fn style(&self) -> TileStyle {
        TileStyle::Bubble
    }

    /// Return the icons to be used with the [`TileStyle::Icon`] style.
    fn icon(&self) -> nvim_oxi::Result<String> {
        Ok(String::new())
    }

    /// Return the content to be render of this tile.
    fn content(&self) -> nvim_oxi::Result<String>;

    /// Return the highlight group name to be use for the tile.
    ///
    /// The default highlight color should be set by using
    /// the [`Tile::highlight_opt`] function and updating the highlights color using the
    /// [`Tile::update_highlight`] method. This should return a group that the background is colored
    fn highlight_name(&self) -> nvim_oxi::Result<String>;

    /// Return the name for the reverse highlights group.
    fn highlight_rev_name(&self, norm_hl: String) -> nvim_oxi::Result<String> {
        Ok(format!("{norm_hl}Rev"))
    }
    /// Return the name for the separator highlights group.
    /// [`TileStyle::Icon`].
    fn highlight_sep_name(&self, norm_hl: String) -> nvim_oxi::Result<String> {
        Ok(format!("{norm_hl}Sep"))
    }

    /// Return the highlight group option that is use by default.
    ///
    /// This function is only run once on
    /// setup and never again. Use [`Tile::update_highlight`] to update the highlight group later.
    /// [`TileStyle::Icon`] should just return the colored background.
    fn highlight_opt(&self) -> HighlightOpt;

    /// Setup other information/components that a tiles might need.
    fn setup(&self) -> nvim_oxi::Result<()> {
        Ok(())
    }

    /// Update any field that the tile might need to compute it's value.
    fn update(&mut self) -> nvim_oxi::Result<()> {
        Ok(())
    }

    /// Update the highlight group based on the old highlight group.
    fn update_highlight(&self, old_opt: HighlightOpt) -> nvim_oxi::Result<HighlightOpt> {
        Ok(old_opt)
    }
}

/// Type alias for a collection of [`Tile`] as well as their [`HighlightOpt`] caches
pub type Tiles = Vec<(Box<dyn Tile>, HighlightOpt)>;

/// A line of [`Tile`] separated into section to be align.
///
/// A line is separated into 5 sections:
/// - Left: A left aligned section.
/// - Left Center: A center align section render to the left of the absolute center.
/// - Center: A center align section render at the absolute center.
/// - Right Center: A center align section render to the right of the absolute center.
/// - Right: A right aligned section
///
/// Each section of the line are render then join together with space.
#[derive(Default)]
pub struct Line {
    not_setup: bool,
    left: Tiles,
    left_center: Tiles,
    center: Tiles,
    right_center: Tiles,
    right: Tiles,
    exclude_ft: Vec<&'static str>,
}

impl Line {
    /// Create a new [`Line`] builder to add tile to.
    #[must_use]
    pub fn new() -> Self {
        Line {
            not_setup: true,
            ..Default::default()
        }
    }

    fn set_hl(tile: &dyn Tile, hl_opt: HighlightOpt) -> nvim_oxi::Result<()> {
        let norm_hl = tile.highlight_name()?;
        match tile.style() {
            TileStyle::Bubble => {
                set_hl(
                    tile.highlight_name()?,
                    hl_opt.clone().fg_if_none(STATUS_LINE_BG),
                )?;
                set_hl(
                    tile.highlight_rev_name(norm_hl.clone())?,
                    hl_opt.reverse_fg_bg().bg(STATUS_LINE_BG),
                )?;
            }
            TileStyle::Icon => {
                set_hl(
                    tile.highlight_name()?,
                    hl_opt.clone().fg_if_none(STATUS_LINE_BG),
                )?;
                set_hl(
                    tile.highlight_rev_name(norm_hl.clone())?,
                    hl_opt.clone().reverse_fg_bg().bg(STATUS_LINE_FG),
                )?;
                set_hl(
                    tile.highlight_sep_name(norm_hl.clone())?,
                    hl_opt.reverse_fg_bg().bg(STATUS_LINE_BG),
                )?;
            }
        }
        Ok(())
    }

    /// Don't call this method manually
    fn setup(&mut self) -> nvim_oxi::Result<()> {
        fn setup_section(section: &Tiles) -> nvim_oxi::Result<()> {
            if !section.is_empty() {
                for (tile, _) in section {
                    Line::set_hl(&**tile, tile.highlight_opt())?;
                    tile.setup()?;
                }
            }
            Ok(())
        }

        setup_section(&self.left)?;
        setup_section(&self.left_center)?;
        setup_section(&self.center)?;
        setup_section(&self.right_center)?;
        setup_section(&self.right)?;

        self.not_setup = false;

        Ok(())
    }

    /// Return the rendered version of this line.
    pub fn render(&mut self) -> nvim_oxi::Result<String> {
        fn render_section(section: &mut Tiles) -> nvim_oxi::Result<String> {
            if section.is_empty() {
                return Ok(String::new());
            }
            let mut sections: Vec<String> = vec![];

            for tile in section {
                tile.0.update()?;

                let content = tile.0.content()?;

                if content.is_empty() {
                    continue;
                }

                let norm = tile.0.highlight_name()?;
                let rev = tile.0.highlight_rev_name(norm.clone())?;

                // We can use clone here without much performance issue because all of the
                // highlights group shouldn't be link to anything so we never have to clone a
                // string.
                // This code only make the set_hl call when it is actually necessary for better
                // performance by caching the value.
                let old_hl = tile.1.clone();
                tile.1 = tile.0.update_highlight(tile.1.clone())?;
                if old_hl != tile.1 {
                    Line::set_hl(&*tile.0, tile.1.clone())?;
                }

                let tile = match tile.0.style() {
                    TileStyle::Bubble => format!("%#{rev}#%#{norm}#{content}%#{rev}#%*",),
                    TileStyle::Icon => {
                        assert!(!tile.0.icon()?.is_empty());
                        let sep = tile.0.highlight_sep_name(norm.clone())?;

                        format!(
                            "%#{sep}#%#{norm}#{icon} %#{rev}# {content}%*%*",
                            icon = tile.0.icon()?,
                        )
                    }
                };

                sections.push(tile);
            }

            Ok(sections.join(" "))
        }

        /// Pad the left and right section with space so that the center section is actually center
        /// align to the window.
        #[allow(clippy::needless_pass_by_value)]
        fn equalize(left: String, right: String) -> nvim_oxi::Result<(String, String)> {
            // Janky fix for nvim_oxi bug:
            // https://github.com/noib3/nvim-oxi/issues/267
            // TODO: update nvim_oxi when this bug is fix
            let (left_len, right_len) = (
                if left.is_empty() {
                    0
                } else {
                    eval_status(&left)?.width
                },
                if right.is_empty() {
                    0
                } else {
                    eval_status(&right)?.width
                },
            );
            Ok((
                format!(
                    "{}{}",
                    left,
                    " ".repeat(right_len.saturating_sub(left_len).try_into().unwrap())
                ),
                format!(
                    "{}{}",
                    " ".repeat(left_len.saturating_sub(right_len).try_into().unwrap()),
                    right,
                ),
            ))
        }

        if self.not_setup {
            self.setup()?;
        }

        if self
            .exclude_ft
            .contains(&get_option::<String>("ft")?.as_str())
        {
            return Ok(String::new());
        }

        let (left, lcent, cent, rcent, right) = (
            render_section(&mut self.left)?,
            render_section(&mut self.left_center)?,
            render_section(&mut self.center)?,
            render_section(&mut self.right_center)?,
            render_section(&mut self.right)?,
        );

        let (left, right) = equalize(left, right)?;
        // fliped because the padding direction is reversed on center
        let (rcent, lcent) = equalize(rcent, lcent)?;

        Ok(format!("{left}%={lcent} {cent} {rcent}%={right}",))
    }

    /// Add a tile to the left section of this line.
    pub fn add_left<T>(&mut self, tile: T)
    where
        T: Tile + 'static,
    {
        let opt = tile.highlight_opt();
        self.left.push((Box::new(tile), opt));
    }

    /// Add a tile to the left center section of this line.
    ///
    /// This tile will be render left of the absolute center.
    #[allow(dead_code)]
    pub fn add_left_center<T>(&mut self, tile: T)
    where
        T: Tile + 'static,
    {
        let opt = tile.highlight_opt();
        self.left_center.push((Box::new(tile), opt));
    }

    /// Add a tile to the center section of this line.
    ///
    /// This tile will be render at the absolute center.
    pub fn add_center<T>(&mut self, tile: T)
    where
        T: Tile + 'static,
    {
        let opt = tile.highlight_opt();
        self.center.push((Box::new(tile), opt));
    }

    /// Add a tile to the right center section of this line.
    ///
    /// This tile will be render right of the absolute center.
    pub fn add_right_center<T>(&mut self, tile: T)
    where
        T: Tile + 'static,
    {
        let opt = tile.highlight_opt();
        self.right_center.push((Box::new(tile), opt));
    }

    /// Add a tile to the right section of this line.
    pub fn add_right<T>(&mut self, tile: T)
    where
        T: Tile + 'static,
    {
        let opt = tile.highlight_opt();
        self.right.push((Box::new(tile), opt));
    }

    /// Add a file type to be excluded when rendering
    pub fn exclude_ft(&mut self, ft: &'static str) {
        self.exclude_ft.push(ft);
    }
}

/// Helper to evaluate a statusline string.
pub fn eval_status(str: impl Into<String>) -> nvim_oxi::Result<StatuslineInfos> {
    Ok(nvim_oxi::api::eval_statusline(
        &str.into(),
        &nvim_oxi::api::opts::EvalStatuslineOpts::default(),
    )?)
}
