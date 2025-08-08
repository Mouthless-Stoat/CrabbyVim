use mlua::ObjectLike;

use crate::keymaps::Action;
use crate::plugins::Plugins;
use crate::theme::Color;
use crate::{icons, require, table};

use crate::lazy::{LazyKey, LazyLoad, LazyPlugin};

pub fn plugins() -> Plugins {
    // TODO: replace this lua spam with rust function to be more "authentic"
    Ok(vec![
        LazyPlugin::new("folke/snacks.nvim")
            .opts(table! {
                picker = picker_config()?
            })
            .lazy_load(
                LazyLoad::new(false)
                    .add_key(LazyKey::new("<Leader>sf").action(snack_picker("files")))
                    .add_key(LazyKey::new("<Leader>st").action(snack_picker("grep")))
                    .add_key(
                        LazyKey::new("<Leader>ss").action(snack_picker("lsp_workspace_symbols")),
                    )
                    .add_key(LazyKey::new("<Leader>u").action(snack_picker("undo")))
                    .add_key(LazyKey::new("<Leader>g").action(Action::Fn(Box::new(|| {
                        Ok(require("snacks")?
                            .get::<mlua::Table>("lazygit")?
                            .call_function::<()>("open", ())?)
                    })))),
            ),
    ])
}

fn snack_picker(picker: &'static str) -> Action {
    Action::Fn(Box::new(|| {
        Ok(require("snacks")?
            .get::<mlua::Table>("picker")?
            .call_function::<()>(picker, ())?)
    }))
}

fn picker_config() -> nvim_oxi::Result<mlua::Table> {
    use icons::*;
    Ok(table! {
        prompt = format!(" {MAGNIFYING_GLASS} "),
        icons = table! {
            kinds = table! {
                Text = icons::TEXT,
                Method = icons::METHOD,
                Function = icons::FUNCTION,
                Constructor = icons::CONSTRUCTOR,

                Field = icons::FIELD,
                Variable = icons::VARIABLE,
                Property = icons::PROPERTY,

                Class = icons::CLASS,
                Interface = icons::INTERFACE,
                Struct = icons::STRUCT,
                Module = icons::MODULE,

                Unit = icons::UNIT,
                Value = icons::VALUE,
                Enum = icons::ENUM,
                EnumMember = icons::ENUM_MEMBER,

                Keyword = icons::KEYWORD,
                Constant = icons::CONSTANT,

                Snippet = icons::SNIPPET,
                Color = icons::COLOR,
                File = icons::FILE,
                Reference = icons::REFERENCE,
                Folder = icons::FOLDER,
                Event = icons::EVENT,
                Operator = icons::OPERATOR,
                TypeParameter = icons::TYPE_PARAMETER
            }
        }
    })
}
