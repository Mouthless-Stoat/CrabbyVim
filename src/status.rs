use std::fmt::Debug;

use crate::options::set_option;
use crate::theme::{HighlightOpt, get_hl, set_hl};

pub fn configure() -> nvim_oxi::Result<()> {
    set_hl(
        "statusline",
        HighlightOpt::with_bg(crate::theme::Color::Bg1).fg(crate::theme::Color::Bg2),
    )?;

    set_option("laststatus", 3)?;

    let mut statusline = Line::new();

    nvim_oxi::mlua::lua().globals().set(
        "statusline",
        nvim_oxi::mlua::lua().create_function(move |_, ()| {
            Ok(statusline.render().expect("Can't render statusline"))
        })?,
    )?;

    set_option("statusline", "%!v:lua.statusline()")?;

    Ok(())
}

type Tiles = Vec<Box<dyn Tile>>;
enum TileStyle {
    Bubble,
    Icon,
}

trait Tile: Debug {
    /// Return the icons to be used with the [`TileStyle::Icon`] style
    fn icon(&self) -> nvim_oxi::Result<String> {
        Ok(String::new())
    }

    /// This just compute the highlights name. The default highlight color should be set by using
    /// the `highlight_opt()` function and updating the highlights color using the
    /// `update_highlight()` method. This should return a group that the background is colored
    /// and the foreground being normal
    fn highlight_name(&self) -> nvim_oxi::Result<&'static str>;
    /// This return the highlight group that is use by default. This function is only run once on
    /// setup and never again. Use `update_highlight` to update the highlight group later.
    fn highlight_opt(&self) -> HighlightOpt;

    /// Function to update the highlight group
    fn update_highlight(&self, old_opt: HighlightOpt) -> nvim_oxi::Result<HighlightOpt> {
        Ok(old_opt)
    }

    fn content(&self) -> nvim_oxi::Result<String>;

    fn style(&self) -> TileStyle {
        TileStyle::Bubble
    }
}

#[derive(Debug, Default)]
struct Line {
    left: Tiles,
    center: Tiles,
    right: Tiles,
}

impl Line {
    fn new() -> Self {
        Line {
            ..Default::default()
        }
    }

    fn setup(&self) -> nvim_oxi::Result<()> {
        fn setup_section(section: &Tiles) -> nvim_oxi::Result<()> {
            for tile in section {
                set_hl(tile.highlight_name()?, tile.highlight_opt())?;
            }
            Ok(())
        }

        setup_section(&self.left)?;
        setup_section(&self.center)?;
        setup_section(&self.right)?;

        Ok(())
    }

    fn render(&self) -> nvim_oxi::Result<String> {
        fn process_section(section: &Tiles) -> nvim_oxi::Result<String> {
            Ok(section
                .iter()
                .map(|tile| {
                    // highlight group name
                    let norm = tile.highlight_name()?;
                    let rev = format!("{norm}Rev");

                    set_hl(
                        tile.highlight_name()?,
                        tile.update_highlight(get_hl(tile.highlight_name()?)?)?,
                    )?;

                    // set the separator highlights to be opposite of normal hl
                    let main_hl = get_hl(norm)?;
                    let mut hl = HighlightOpt::default();

                    if let Some(fg) = main_hl.fg {
                        hl = hl.bg(fg);
                    }
                    if let Some(bg) = main_hl.bg {
                        hl = hl.fg(bg);
                    }

                    set_hl(&rev, hl)?;

                    // push the tile to the statusline
                    match tile.style() {
                        TileStyle::Bubble => Ok(format!(
                            "%#{rev}#%#{norm}#{cont}%#{rev}#%*",
                            cont = tile.content()?
                        )),
                        TileStyle::Icon => {
                            assert!(!tile.icon()?.is_empty());

                            Ok(format!(
                                "%#{rev}#%#{norm}#{icon} %#{rev}# {cont}%*%*",
                                icon = tile.icon()?,
                                cont = tile.content()?
                            ))
                        }
                    }
                })
                .collect::<nvim_oxi::Result<Vec<_>>>()?
                .join(" "))
        }

        Ok(format!(
            "{}%={}%={}",
            process_section(&self.left)?,
            process_section(&self.center)?,
            process_section(&self.right)?
        ))
    }

    fn add_left<T>(&mut self, tile: T)
    where
        T: Tile + 'static,
    {
        self.left.push(Box::new(tile));
    }

    fn add_center<T>(&mut self, tile: T)
    where
        T: Tile + 'static,
    {
        self.center.push(Box::new(tile));
    }

    fn add_right<T>(&mut self, tile: T)
    where
        T: Tile + 'static,
    {
        self.right.push(Box::new(tile));
    }
}
