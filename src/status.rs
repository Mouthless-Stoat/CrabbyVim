use crate::options::set_option;
use crate::theme::{Color, HighlightOpt, set_hl};

mod tiles;
use tiles::*;

const STATUS_LINE_BG: Color = crate::theme::Color::Bg1;
const STATUS_LINE_FG: Color = crate::theme::Color::Bg2;

pub fn configure() -> nvim_oxi::Result<()> {
    set_hl(
        "statusline",
        HighlightOpt::with_bg(STATUS_LINE_BG).fg(STATUS_LINE_FG),
    )?;

    set_option("laststatus", 3)?;

    let mut statusline = Line::new();

    statusline.add_left(Mode::new());
    statusline.add_left(Cwd::new());
    statusline.add_left(Git);

    statusline.add_right(Loc);
    statusline.add_right(Zoom);

    statusline.setup()?;

    nvim_oxi::mlua::lua().globals().set(
        "statusline",
        nvim_oxi::mlua::lua().create_function_mut(move |_, ()| {
            Ok(statusline.render().expect("Can't render statusline"))
        })?,
    )?;

    set_option("statusline", "%!v:lua.statusline()")?;

    Ok(())
}

enum TileStyle {
    Bubble,
    Icon,
}

/// Trait for a tile to implement. Each tile need to implement:
/// [`Tile::content()`], [`Tile::highlight_name`] and [`Tile::highlight_opt`].
/// - [`Tile::content`] return what need to be render within each tile
/// - [`Tile::highlight_name`] return the highlight group that will be use to highlight the tile.
///   This should only color the the background of the highlight group, the foreground and everything
///   else will be calculate by the render.
/// - [`Tile::highlight_opt`] return the option to setup the highlight group for the first time.
///
/// Other important method like [`Tile::setup`] and [`Tile::update`] use to create or update the
/// value.
trait Tile {
    fn style(&self) -> TileStyle {
        TileStyle::Bubble
    }

    /// Return the icons to be used with the [`TileStyle::Icon`] style
    fn icon(&self) -> nvim_oxi::Result<String> {
        Ok(String::new())
    }

    fn content(&self) -> nvim_oxi::Result<String>;

    /// This just compute the highlights name. The default highlight color should be set by using
    /// the [`Tile::highlight_opt`] function and updating the highlights color using the
    /// [`Tile::update_highlight`] method. This should return a group that the background is colored
    /// and the foreground being normal
    fn highlight_name(&self) -> nvim_oxi::Result<&'static str>;
    /// This method return the name for the reverse highlights group.
    fn highlight_rev_name(&self, norm_hl: &'static str) -> nvim_oxi::Result<String> {
        Ok(format!("{norm_hl}Rev"))
    }
    /// This method return the name for the separator highlights group used for
    /// [`TileStyle::Icon`].
    fn highlight_sep_name(&self, norm_hl: &'static str) -> nvim_oxi::Result<String> {
        Ok(format!("{norm_hl}Sep"))
    }

    /// This return the highlight group that is use by default. This function is only run once on
    /// setup and never again. Use [`Tile::update_highlight`] to update the highlight group later.
    /// [`TileStyle::Icon`] should just return the colored background.
    fn highlight_opt(&self) -> HighlightOpt;

    /// This setup other highlights group that the tile might need.
    fn setup(&self) -> nvim_oxi::Result<()> {
        Ok(())
    }

    /// This is run to update any field that the tile might need to update to use.
    fn update(&mut self) -> nvim_oxi::Result<()> {
        Ok(())
    }

    /// Function to update the highlight group
    fn update_highlight(&self, old_opt: HighlightOpt) -> nvim_oxi::Result<HighlightOpt> {
        Ok(old_opt)
    }
}

type Tiles = Vec<(Box<dyn Tile>, HighlightOpt)>;
#[derive(Default)]
struct Line {
    not_setup: bool,
    left: Tiles,
    center: Tiles,
    right: Tiles,
}

impl Line {
    fn new() -> Self {
        Line {
            not_setup: true,
            ..Default::default()
        }
    }

    fn set_hl(tile: &dyn Tile, hl_opt: HighlightOpt) -> nvim_oxi::Result<()> {
        let norm_hl = tile.highlight_name()?;
        match tile.style() {
            TileStyle::Bubble => {
                set_hl(tile.highlight_name()?, hl_opt.clone().fg(STATUS_LINE_BG))?;
                set_hl(
                    tile.highlight_rev_name(norm_hl)?,
                    hl_opt.fg(STATUS_LINE_BG).reverse_fg_bg(),
                )?;
            }
            TileStyle::Icon => {
                set_hl(tile.highlight_name()?, hl_opt.clone().fg(STATUS_LINE_BG))?;
                set_hl(
                    tile.highlight_rev_name(norm_hl)?,
                    hl_opt.clone().reverse_fg_bg().bg(STATUS_LINE_FG),
                )?;
                set_hl(
                    tile.highlight_sep_name(norm_hl)?,
                    hl_opt.reverse_fg_bg().bg(STATUS_LINE_BG),
                )?;
            }
        }
        Ok(())
    }

    /// Don't call this method twice
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
        setup_section(&self.center)?;
        setup_section(&self.right)?;
        self.right.reverse();

        self.not_setup = false;

        Ok(())
    }

    fn render(&mut self) -> nvim_oxi::Result<String> {
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
                let rev = tile.0.highlight_rev_name(norm)?;

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
                        let sep = tile.0.highlight_sep_name(norm)?;

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

        if self.not_setup {
            self.setup()?;
        }

        let (left, cent, right) = (
            render_section(&mut self.left)?,
            render_section(&mut self.center)?,
            render_section(&mut self.right)?,
        );

        // pad the left and right section with space so that the center section is actually center
        // align to the window.
        let (left_len, right_len) = (
            nvim_oxi::api::eval_statusline(
                &left,
                &nvim_oxi::api::opts::EvalStatuslineOpts::default(),
            )?
            .str
            .len(),
            nvim_oxi::api::eval_statusline(
                &right,
                &nvim_oxi::api::opts::EvalStatuslineOpts::default(),
            )?
            .str
            .len(),
        );

        let left = format!("{}{}", left, " ".repeat(right_len.saturating_sub(left_len)));
        let right = format!(
            "{}{}",
            " ".repeat(left_len.saturating_sub(right_len)),
            right,
        );

        Ok(format!("{left}%={cent}%={right}",))
    }

    fn add_left<T>(&mut self, tile: T)
    where
        T: Tile + 'static,
    {
        let opt = tile.highlight_opt();
        self.left.push((Box::new(tile), opt));
    }

    fn add_center<T>(&mut self, tile: T)
    where
        T: Tile + 'static,
    {
        let opt = tile.highlight_opt();
        self.center.push((Box::new(tile), opt));
    }

    fn add_right<T>(&mut self, tile: T)
    where
        T: Tile + 'static,
    {
        let opt = tile.highlight_opt();
        self.right.push((Box::new(tile), opt));
    }
}

pub fn eval_status(str: impl Into<String>) -> nvim_oxi::Result<StatuslineInfos> {
    Ok(nvim_oxi::api::eval_statusline(
        &str.into(),
        &nvim_oxi::api::opts::EvalStatuslineOpts::default(),
    )?)
}
