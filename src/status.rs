use crate::options::set_option;
use crate::theme::{HighlightOpt, configure_highlights, get_hl, set_hl};

pub fn configure() -> nvim_oxi::Result<()> {
    highlights()?;

    let statusline = Line {
        left: vec![],
        center: vec![],
        right: vec![],
    };

    nvim_oxi::mlua::lua().globals().set(
        "MkStatus",
        nvim_oxi::mlua::lua().create_function(move |_, ()| {
            Ok(statusline.render().expect("Can't render statusline"))
        })?,
    )?;

    set_option("statusline", "%!v:lua.MkStatus()")?;

    Ok(())
}

type Tiles = Vec<Box<dyn StatusTile>>;
enum TileStyle {
    Bubble,
    Icon,
}

trait StatusTile {
    fn icons(&self) -> &'static str;
    fn highlights(&self) -> nvim_oxi::Result<&'static str>;
    fn content(&self) -> nvim_oxi::Result<String>;
    fn style(&self) -> TileStyle {
        TileStyle::Bubble
    }
}

struct Line {
    left: Tiles,
    center: Tiles,
    right: Tiles,
}

impl Line {
    fn render(&self) -> nvim_oxi::Result<String> {
        fn process_section(section: &Tiles) -> nvim_oxi::Result<String> {
            Ok(section
                .iter()
                .map(|tile| match tile.style() {
                    TileStyle::Bubble => {
                        // highlight group name
                        let main = tile.highlights()?;
                        let sep = format!("{main}Sep");

                        // set the separator highlights to be opposite of normal hl
                        let main_hl = get_hl(main)?;
                        let mut hl = HighlightOpt::default();

                        if let Some(fg) = main_hl.fg {
                            hl = hl.fg(fg);
                        }
                        if let Some(bg) = main_hl.bg {
                            hl = hl.bg(bg);
                        }

                        set_hl(format!("{}Sep", tile.highlights()?), hl)?;

                        // push the tile to the statusline
                        Ok(format!("%#{sep}#%#{main}#{}%#{sep}#%*", tile.content()?))
                    }
                    TileStyle::Icon => todo!(),
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
}

pub fn highlights() -> nvim_oxi::Result<()> {
    use crate::theme::Color::*;

    configure_highlights(vec![
        ("StatusMode", HighlightOpt::with_fg(Blue).bg(Bg1)),
        ("StatusCwd", HighlightOpt::with_fg(Blue).bg(Bg1)),
    ])
}
