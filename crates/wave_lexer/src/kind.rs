use std::fmt;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
#[non_exhaustive]
pub enum Kind {
    Undetermined,
    #[default]
    Eof,
    Ident,
    NewLine,
    Decimal,
    WhiteSpace,
    Comment,
    MultiLineComment,
    //Declaration
    Let,
    Const,

    // Arithmetic Binary Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Star2,

    Eq,
    Comma,
    Semicolon,
    Null,
    True,
    False,
    Str,

    // Brackets
    RCurly,
}

use self::Kind::*;

impl Kind {
    pub fn is_eof(self) -> bool {
        matches!(self, Eof)
    }

    pub fn is_literal(self) -> bool {
        matches!(self, Null | True | False | Str) || self.is_number()
    }

    pub fn is_number(self) -> bool {
        matches!(self, Decimal)
    }

    pub fn is_assignment_operator(self) -> bool {
        matches!(self, Eq)
    }

    pub fn to_str(self) -> &'static str {
        match self {
            Undetermined => "Unknown",
            Eof => "EOF",
            Decimal => "Decimal",
            NewLine => "\n",
            Ident => "Identifier",
            WhiteSpace => " ",
            Comment => "\\",
            MultiLineComment => "/** */",
            Let => "let",
            Eq => "=",
            Comma => ",",
            Semicolon => ";",
            Null => "null",
            True => "true",
            False => "false",
            Str => "String",
            Const => "const",
            RCurly => "}",
            Plus => "+",
            Minus => "-",
            Star => "*",
            Slash => "/",
            Percent => "%",
            Star2 => "**",
        }
    }

    pub fn is_binding_identifier(self) -> bool {
        self.is_identifier()
    }

    pub fn is_identifier(self) -> bool {
        self.is_identifier_name()
    }

    pub fn is_identifier_name(self) -> bool {
        matches!(self, Ident)
    }

    #[rustfmt::skip]
    pub fn is_binary_operator(self) -> bool {
        matches!(self, Plus | Minus | Star | Slash | Percent | Star2)
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
