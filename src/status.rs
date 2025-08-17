use std::fmt::Debug;

use crate::options::set_option;
use crate::theme::{HighlightOpt, configure_highlights, get_hl, set_hl};

pub fn configure() -> nvim_oxi::Result<()> {
    highlights()?;

    set_option("laststatus", 3)?;

    let mut statusline = Line::new();

    nvim_oxi::mlua::lua().globals().set(
        "MkStatus",
        nvim_oxi::mlua::lua().create_function(move |_, ()| {
            Ok(statusline.render().expect("Can't render statusline"))
        })?,
    )?;

    set_option("statusline", "%!v:lua.MkStatus()")?;

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

    ///
    /// This should return the background being colored and foreground being normal
    fn highlights(&self) -> nvim_oxi::Result<&'static str>;

    /// Function to update the highlight group
    fn update_highlights(&self, old_opt: HighlightOpt) -> nvim_oxi::Result<HighlightOpt> {
        Ok(old_opt)
    }

    fn content(&self) -> nvim_oxi::Result<String>;

    fn style(&self) -> TileStyle {
        TileStyle::Bubble
    }
}

#[derive(Debug)]
struct Line {
    left: Tiles,
    center: Tiles,
    right: Tiles,
}

impl Line {
    fn new() -> Self {
        Line {
            left: vec![],
            center: vec![],
            right: vec![],
        }
    }

    fn render(&self) -> nvim_oxi::Result<String> {
        fn process_section(section: &Tiles) -> nvim_oxi::Result<String> {
            Ok(section
                .iter()
                .map(|tile| {
                    // highlight group name
                    let norm = tile.highlights()?;
                    let rev = format!("{norm}Rev");

                    tile.update_highlights(get_hl(tile.highlights()?)?)?;

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
                            assert!(tile.icon()?.is_empty());

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

pub fn highlights() -> nvim_oxi::Result<()> {
    use crate::theme::Color::*;

    configure_highlights(vec![
        ("StatusMode", HighlightOpt::with_bg(Blue).fg(Bg2)),
        ("StatusCwd", HighlightOpt::with_bg(Blue).fg(Bg1)),
        ("StatusLine", HighlightOpt::with_bg(Bg1).fg(Bg2)),
    ])
}
