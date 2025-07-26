use super::{Color, HighlightOpt, set_hl};

pub fn configure_highlight() -> nvim_oxi::Result<()> {
    use SyntaxGroup::*;

    // Recommended neovim highlight group
    set_hl(Comment, HighlightOpt::default().fg(Color::Gray))?;

    set_hl(String, HighlightOpt::default().fg(Color::Green))?;
    set_hl(Number, HighlightOpt::default().fg(Color::Orange))?;
    set_hl(Float, HighlightOpt::link(Number))?;
    set_hl(Boolean, HighlightOpt::default().fg(Color::Orange))?;
    set_hl(Character, HighlightOpt::default().fg(Color::Orange))?;
    set_hl(Structure, HighlightOpt::default().fg(Color::Yellow))?;

    set_hl(Identifier, HighlightOpt::default().fg(Color::Red))?;
    set_hl(Constant, HighlightOpt::default().fg(Color::Yellow))?;
    set_hl(Member, HighlightOpt::default().fg(Color::Cyan))?;
    set_hl(Builtin, HighlightOpt::default().fg(Color::Purple))?;

    set_hl(Function, HighlightOpt::default().fg(Color::Blue))?;
    set_hl(Statement, HighlightOpt::default().fg(Color::Purple))?;
    set_hl(Keyword, HighlightOpt::default().fg(Color::Purple))?;

    set_hl(Preproc, HighlightOpt::default().fg(Color::Purple))?;
    set_hl(Type, HighlightOpt::default().fg(Color::Orange))?;

    set_hl(Operator, HighlightOpt::default().fg(Color::Cyan))?;
    set_hl(Special, HighlightOpt::default().fg(Color::Pink).italic())?;
    set_hl(Delimiter, HighlightOpt::default().fg(Color::Gray))?;

    set_hl("@variable", HighlightOpt::link(Identifier))?;
    set_hl("@variable.member", HighlightOpt::link(Member))?;
    set_hl("@variable.builtin", HighlightOpt::link(Builtin))?;

    set_hl("@type.builtin", HighlightOpt::link(Type))?;

    set_lsp_hl("property", HighlightOpt::link(Member))?;
    set_lsp_hl("modifier", HighlightOpt::link(Keyword))?;

    set_lsp_hl("struct", HighlightOpt::link(Structure))?;
    set_lsp_hl("enum", HighlightOpt::link(Structure))?;
    set_lsp_hl(
        "enumMember",
        HighlightOpt::default().fg(Color::Yellow).italic(),
    )?;

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

impl From<SyntaxGroup> for &'static str {
    fn from(val: SyntaxGroup) -> Self {
        match val {
            SyntaxGroup::Comment => "Comment",
            SyntaxGroup::String => "String",
            SyntaxGroup::Number => "Number",
            SyntaxGroup::Float => "Float",
            SyntaxGroup::Boolean => "Boolean",
            SyntaxGroup::Character => "Character",
            SyntaxGroup::Structure => "Struture",
            SyntaxGroup::Identifier => "Identifier",
            SyntaxGroup::Constant => "Constant",
            SyntaxGroup::Member => "Member",
            SyntaxGroup::Builtin => "Builtin",
            SyntaxGroup::Function => "Function",
            SyntaxGroup::Statement => "Statement",
            SyntaxGroup::Keyword => "Keyword",
            SyntaxGroup::Preproc => "Preproc",
            SyntaxGroup::Type => "Type",
            SyntaxGroup::Operator => "Operator",
            SyntaxGroup::Special => "Special",
            SyntaxGroup::Delimiter => "Delimiter",
        }
    }
}

fn set_lsp_hl(hl: &str, opt: HighlightOpt) -> nvim_oxi::Result<()> {
    set_hl(format!("@lsp.type.{hl}").as_str(), opt)?;
    Ok(())
}
