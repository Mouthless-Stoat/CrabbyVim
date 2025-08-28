use mlua::{ObjectLike, Table};
use nvim_oxi::api::types::LogLevel;
use rand::random_range;

use crate::theme::{HighlightOpt, configure_highlights};
use crate::{func, icons, lua_vec, require, table, vim_notify};

pub const CRAB: &str = r#"
   __       __   
  / <'     '> \  
 (  / @   @ \  ) 
  \(_ _\_/_ _)/  
(\ '-/     \-' /)
 "===\     /===" 
  .==')___('==.  
 ' .='     `=. ' 
"#;

pub const NEO: &str = r"
                 
                 
      __         
   /\ \ \___  ___
  /  \/ / _ \/ _ 
 / /\  |  __| (_)
 \_\ \/ \___|\___/
                 
";

pub const VIM: &str = r"
                   
                   
                   
/\   /(_)_ __ ___  
\ \ / | | '_ ` _ \ 
 \ V /| | | | | | |
 \_/ |_|_| |_| |_|
                   
";

macro_rules! text {
    ($text:expr, $hl:expr) => {
        table! { [1] = $text, ["hl"] = $hl }
    };
}

pub fn config() -> nvim_oxi::Result<mlua::Table> {
    let tagline: Vec<mlua::Table> = match random_range(0..=3) {
        0 => vec![text!(
            "Mouse is for nerd, Keyboard rank supreme",
            "SnacksDashboardFooter"
        )],
        1 => vec![
            text!("Rewrite it in ", "SnacksDashboardFooter"),
            text!("Rust", "SnacksDashboardOrangeItalic"),
        ],
        2 => vec![
            text!("In ", "SnacksDashboardFooter"),
            text!("Rust", "SnacksDashboardOrangeItalic"),
            text!(" we trust", "SnacksDashboardFooter"),
        ],
        3 => vec![
            text!("Crabby", "SnacksDashboardOrangeItalic"),
            text!("Vim", "SnacksDashboardGreenItalic"),
            text!(" a neovim config written in ", "SnacksDashboardFooter"),
            text!("rust", "SnacksDashboardOrangeItalic"),
        ],
        _ => unreachable!(),
    };

    Ok(table! {
        preset = table! {
            keys = vec![
                table!{ hidden = true, key = "f", action = "<leader>sf" },
                table!{ hidden = true, key = "s", action = ":Scratch" },
                table!{ hidden = true, key = "u", action = ":Lazy update" },
                table!{ hidden = true, key = "q", action = ":qa" }
            ]
        },
        sections = lua_vec! [
            table!{ align = "center", text = header()? },
            table!{ align = "center", text = tagline, padding = 1 },
            table!{
                align = "center",
                text = vec![
                    text!(format!(" {} Files ", icons::FOLDER), "SnacksDashboardRed"),
                    text!(format!(" {} Scratch ", icons::FILE), "SnacksDashboardGreen"),
                    text!(format!(" {} Update ", icons::UPDATE), "SnacksDashboardYellow"),
                    text!(format!(" {} Quit ", icons::EXIT), "SnacksDashboardPurple")
                ]
            },

            table!{ section = "keys", padding = 1 },
            table!{
                title = text!("Recents", "SnacksDashboardGreen"),
                icon = text!(icons::FILE, "SnacksDashboardGreen"),
                section = "recent_files", limit = 5, indent = 2, padding = 1
            },
            table!{
                title = text!("Projects", "SnacksDashboardBlue"),
                icon = text!(icons::FOLDER, "SnacksDashboardBlue"),
                section = "projects", limit = 5, indent = 2, padding = 1
            },
            table!{
                icon = text!(icons::GIT_BRANCH, "SnacksDashboardOrange"),
                title = text!("Git Status", "SnacksDashboardOrange"),
                section = "terminal",
                cmd = "git --no-pager diff --stat -B -M -C",
                enabled =
                    func! {
                        || -> bool {
                            require("snacks")?.get::<Table>("git")?.call_function::<bool>("get_root", ())?
                        }
                    },

                height = 10,
                indent = 1,
                padding = 1
            },
            func! {
                || -> mlua::Table {
                    table!{ align = "center", text = startup()? }
                }
            }
        ]
    })
}

pub fn highlights() -> nvim_oxi::Result<()> {
    use crate::theme::Color::*;

    configure_highlights(vec![
        ("SnacksDashboardKey", HighlightOpt::with_fg(Yellow)),
        ("SnacksDashboardFile", HighlightOpt::with_fg(Blue)),
        ("SnacksDashboardIcon", HighlightOpt::with_fg(Blue)),
        (
            "SnacksDashboardFooter",
            HighlightOpt::with_fg(Gray).italic(),
        ),
        // Italic color
        (
            "SnacksDashboardRedItalic",
            HighlightOpt::with_fg(Red).italic(),
        ),
        (
            "SnacksDashboardOrangeItalic",
            HighlightOpt::with_fg(Orange).italic(),
        ),
        (
            "SnacksDashboardYellowItalic",
            HighlightOpt::with_fg(Yellow).italic(),
        ),
        (
            "SnacksDashboardGreenItalic",
            HighlightOpt::with_fg(Green).italic(),
        ),
        (
            "SnacksDashboardCyanItalic",
            HighlightOpt::with_fg(Cyan).italic(),
        ),
        (
            "SnacksDashboardBlueItalic",
            HighlightOpt::with_fg(Blue).italic(),
        ),
        (
            "SnacksDashboardPurpleItalic",
            HighlightOpt::with_fg(Purple).italic(),
        ),
        // Bare color
        ("SnacksDashboardRed", HighlightOpt::with_fg(Red)),
        ("SnacksDashboardOrange", HighlightOpt::with_fg(Orange)),
        ("SnacksDashboardYellow", HighlightOpt::with_fg(Yellow)),
        ("SnacksDashboardGreen", HighlightOpt::with_fg(Green)),
        ("SnacksDashboardCyan", HighlightOpt::with_fg(Cyan)),
        ("SnacksDashboardBlue", HighlightOpt::with_fg(Blue)),
        ("SnacksDashboardPurple", HighlightOpt::with_fg(Purple)),
    ])
}

pub fn header() -> nvim_oxi::Result<mlua::Table> {
    let out = table! {};
    for ((crab, neo), vim) in CRAB.lines().zip(NEO.lines()).zip(VIM.lines()) {
        out.push(text!(crab, "SnacksDashboardOrange"))?;
        out.push(text!(neo, "SnacksDashboardBlue"))?;
        out.push(text!(format!("{vim}\n"), "SnacksDashboardGreen"))?;
    }
    Ok(out)
}

pub fn startup() -> nvim_oxi::Result<Vec<mlua::Table>> {
    let stats = require("lazy")?.call_function::<Table>("stats", ())?;
    let ms = (stats.get::<f64>("startuptime")? * 100.0 + 0.5).floor() / 100.0;

    Ok(vec![
        text!(icons::LIGHTNING, "SnacksDashboardYellow"),
        text!(" Neovim loaded ", "SnacksDashboardFooter"),
        text!(
            stats.get::<i32>("loaded")?.to_string(),
            "SnacksDashboardPurpleItalic"
        ),
        text!("/", "SnacksDashboardPurpleItalic"),
        text!(
            stats.get::<i32>("count")?.to_string(),
            "SnacksDashboardPurpleItalic"
        ),
        text!(" plugins in ", "SnacksDashboardFooter"),
        text!(ms.to_string(), "SnacksDashboardPurpleItalic"),
        text!(" ms ", "SnacksDashboardFooter"),
        text!(icons::LIGHTNING, "SnacksDashboardYellow"),
    ])
}
