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
}

use self::Kind::*;

impl Kind {
    pub fn is_eof(self) -> bool {
        matches!(self, Eof)
    }

    pub fn is_number(self) -> bool {
        matches!(self, Decimal)
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
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
