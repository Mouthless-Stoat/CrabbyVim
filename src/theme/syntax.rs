use super::{Color, HighlightOpt, set_hl};

pub fn highlights() -> nvim_oxi::Result<()> {
    use SyntaxGroup::*;

    // Recommended neovim highlight group
    set_hl(Comment, HighlightOpt::with_fg(Color::Gray))?;

    set_hl(String, HighlightOpt::with_fg(Color::Green))?;
    set_hl(Number, HighlightOpt::with_fg(Color::Orange))?;
    set_hl(Float, HighlightOpt::link(Number))?;
    set_hl(Boolean, HighlightOpt::with_fg(Color::Orange))?;
    set_hl(Character, HighlightOpt::with_fg(Color::Orange))?;
    set_hl(Structure, HighlightOpt::with_fg(Color::Yellow))?;

    set_hl(Identifier, HighlightOpt::with_fg(Color::Red))?;
    set_hl(Constant, HighlightOpt::with_fg(Color::Yellow))?;
    set_hl(Member, HighlightOpt::with_fg(Color::Cyan))?;
    set_hl(Builtin, HighlightOpt::with_fg(Color::Purple))?;

    set_hl(Function, HighlightOpt::with_fg(Color::Blue))?;
    set_hl(Statement, HighlightOpt::with_fg(Color::Purple))?;
    set_hl(Keyword, HighlightOpt::with_fg(Color::Purple))?;

    set_hl(Preproc, HighlightOpt::with_fg(Color::Purple))?;
    set_hl(Type, HighlightOpt::with_fg(Color::Orange))?;

    set_hl(Operator, HighlightOpt::with_fg(Color::Cyan))?;
    set_hl(Special, HighlightOpt::with_fg(Color::Pink).italic())?;
    set_hl(Delimiter, HighlightOpt::with_fg(Color::Gray))?;

    set_hl("@variable", HighlightOpt::link(Identifier))?;
    set_hl("@variable.member", HighlightOpt::link(Member))?;
    set_hl("@variable.builtin", HighlightOpt::link(Builtin))?;

    set_hl("@type.builtin", HighlightOpt::link(Type))?;

    set_lsp_hl("property", HighlightOpt::link(Member))?;
    set_lsp_hl("modifier", HighlightOpt::link(Keyword))?;

    set_lsp_hl("struct", HighlightOpt::link(Structure))?;
    set_lsp_hl("enum", HighlightOpt::link(Structure))?;
    set_lsp_hl("enumMember", HighlightOpt::with_fg(Color::Yellow).italic())?;

    set_hl("rustSigil", HighlightOpt::link(Operator))?;

    Ok(())
}

pub enum SyntaxGroup {
    Comment,
    String,
    Number,
    Float,
    Boolean,
    Character,
    Structure,
    Identifier,
    Constant,
    Member,
    Builtin,
    Function,
    Statement,
    Keyword,
    Preproc,
    Type,
    Operator,
    Special,
    Delimiter,
}

impl From<SyntaxGroup> for String {
    fn from(val: SyntaxGroup) -> Self {
        match val {
            SyntaxGroup::Comment => "Comment".to_string(),
            SyntaxGroup::String => "String".to_string(),
            SyntaxGroup::Number => "Number".to_string(),
            SyntaxGroup::Float => "Float".to_string(),
            SyntaxGroup::Boolean => "Boolean".to_string(),
            SyntaxGroup::Character => "Character".to_string(),
            SyntaxGroup::Structure => "Struture".to_string(),
            SyntaxGroup::Identifier => "Identifier".to_string(),
            SyntaxGroup::Constant => "Constant".to_string(),
            SyntaxGroup::Member => "Member".to_string(),
            SyntaxGroup::Builtin => "Builtin".to_string(),
            SyntaxGroup::Function => "Function".to_string(),
            SyntaxGroup::Statement => "Statement".to_string(),
            SyntaxGroup::Keyword => "Keyword".to_string(),
            SyntaxGroup::Preproc => "Preproc".to_string(),
            SyntaxGroup::Type => "Type".to_string(),
            SyntaxGroup::Operator => "Operator".to_string(),
            SyntaxGroup::Special => "Special".to_string(),
            SyntaxGroup::Delimiter => "Delimiter".to_string(),
        }
    }
}

fn set_lsp_hl(hl: &str, opt: HighlightOpt) -> nvim_oxi::Result<()> {
    set_hl(format!("@lsp.type.{hl}").as_str(), opt)?;
    Ok(())
}
