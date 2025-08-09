use mlua::ObjectLike;

use crate::keymaps::Action;
use crate::{icons, require, table};

pub fn picker(picker: &'static str) -> Action {
    Action::Fn(Box::new(|| {
        Ok(require("snacks")?
            .get::<mlua::Table>("picker")?
            .call_function::<()>(picker, ())?)
    }))
}

pub fn config() -> nvim_oxi::Result<mlua::Table> {
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
