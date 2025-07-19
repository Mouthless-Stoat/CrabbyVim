use super::{Color, HighlightOpt, set_hl};

pub fn syntax_highlight() -> nvim_oxi::Result<()> {
    use SyntaxGroup::*;

    // Recommended neovim highlight group
    set_hl(Comment, HighlightOpt::new().fg(Color::Gray))?;

    set_hl(String, HighlightOpt::new().fg(Color::Green))?;
    set_hl(Number, HighlightOpt::new().fg(Color::Orange))?;
    set_hl(Float, HighlightOpt::link(SyntaxGroup::Number))?;
    set_hl(Boolean, HighlightOpt::new().fg(Color::Orange))?;
    set_hl(Character, HighlightOpt::new().fg(Color::Orange))?;
    set_hl(Structure, HighlightOpt::new().fg(Color::Yellow))?;

    set_hl(Identifier, HighlightOpt::new().fg(Color::Red))?;
    set_hl(Constant, HighlightOpt::new().fg(Color::Yellow))?;
    set_hl(Member, HighlightOpt::new().fg(Color::Cyan))?;
    set_hl(Builtin, HighlightOpt::new().fg(Color::Purple))?;

    set_hl(Function, HighlightOpt::new().fg(Color::Blue))?;
    set_hl(Statement, HighlightOpt::new().fg(Color::Purple))?;
    set_hl(Keyword, HighlightOpt::new().fg(Color::Purple))?;

    set_hl(Preproc, HighlightOpt::new().fg(Color::Purple))?;
    set_hl(Type, HighlightOpt::new().fg(Color::Orange))?;

    set_hl(Operator, HighlightOpt::new().fg(Color::Cyan))?;
    set_hl(Special, HighlightOpt::new().fg(Color::Cyan))?;
    set_hl(Delimiter, HighlightOpt::new().fg(Color::Gray))?;

    set_hl("@variable", HighlightOpt::link(Identifier))?;
    set_hl("@variable.member", HighlightOpt::new().fg(Color::Red))?;
    set_hl("@variable.builtin", HighlightOpt::new().fg(Color::Red))?;

    set_lsp_hl("property", HighlightOpt::link(Member))?;
    set_lsp_hl("modifier", HighlightOpt::link(Keyword))?;

    set_lsp_hl("struct", HighlightOpt::link(Structure))?;
    set_lsp_hl("enum", HighlightOpt::link(Structure))?;

    set_hl("rustSigil", HighlightOpt::link(Operator))?;

    Ok(())
}

enum SyntaxGroup {
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

impl Into<&'static str> for SyntaxGroup {
    fn into(self) -> &'static str {
        match self {
            Self::Comment => "Comment",
            Self::String => "String",
            Self::Number => "Number",
            Self::Float => "Float",
            Self::Boolean => "Boolean",
            Self::Character => "Character",
            Self::Structure => "Struture",
            Self::Identifier => "Identifier",
            Self::Constant => "Constant",
            Self::Member => "Member",
            Self::Builtin => "Builtin",
            Self::Function => "Function",
            Self::Statement => "Statement",
            Self::Keyword => "Keyword",
            Self::Preproc => "Preproc",
            Self::Type => "Type",
            Self::Operator => "Operator",
            Self::Special => "Special",
            Self::Delimiter => "Delimiter",
        }
    }
}

fn set_lsp_hl(hl: &str, opt: HighlightOpt) -> nvim_oxi::Result<()> {
    set_hl(format!("@lsp.type.{hl}").as_str(), opt)?;
    Ok(())
}
