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
    Let,
    Const,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Star2,
    Eq,
    Eq2,
    Comma,
    Semicolon,
    Null,
    True,
    False,
    Str,
    If,
    Else,
    Function,
    Return,
    LParen,
    RParen,
    LCurly,
    RCurly,
    Neq,
    LAngle,
    LtEq,
    RAngle,
    GtEq,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
    Amp2Eq,
    Amp2,
    Amp,
    Pipe2,
    Pipe,
    PipeEq,
    AmpEq,
    Pipe2Eq,
    Star2Eq,
    Caret,
    CaretEq,
    LBrack,
    RBrack,
    Plus2,
    Minus2,
    Bang,
    While,
    Break,
    Continue,
    Dot,
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
            Plus => "+",
            Minus => "-",
            Star => "*",
            Slash => "/",
            Percent => "%",
            Star2 => "**",
            LParen => "(",
            RParen => ")",
            If => "if",
            Else => "else",
            LCurly => "{",
            RCurly => "}",
            Eq2 => "==",
            Function => "function",
            Return => "return",
            Neq => "!=",
            LAngle => "<",
            LtEq => "<=",
            RAngle => ">",
            GtEq => ">=",
            PlusEq => "+=",
            MinusEq => "-=",
            StarEq => "*=",
            SlashEq => "/=",
            PercentEq => "%=",
            Amp2Eq => "&&=",
            Pipe2Eq => "||=",
            Star2Eq => "**=",
            Amp2 => "&&",
            Amp => "&",
            Pipe2 => "||",
            Pipe => "|",
            Caret => "^",
            CaretEq => "^=",
            AmpEq => "&=",
            PipeEq => "|=",
            LBrack => "[",
            RBrack => "]",
            Plus2 => "++",
            Minus2 => "--",
            Bang => "!",
            While => "while",
            Break => "break",
            Continue => "continue",
            Dot => ".",
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

    pub fn is_unary_operator(self) -> bool {
        matches!(self, Minus | Plus | Bang)
    }

    pub fn is_update_operator(self) -> bool {
        matches!(self, Plus2 | Minus2)
    }

    pub fn is_logical_operator(self) -> bool {
        matches!(self, Pipe2 | Amp2)
    }


    #[rustfmt::skip]
    pub fn is_binary_operator(self) -> bool {
        matches!(self, Eq2   | Neq    | LAngle | LtEq  | RAngle  | GtEq |
                       Plus  | Minus  | Star   | Slash | Percent
                                      | Star2)
    }

    #[rustfmt::skip]
    pub fn is_assignment_operator(self) -> bool {
        matches!(self, Eq  | PlusEq   | MinusEq | StarEq  | SlashEq | PercentEq
                           | Pipe2Eq  | Amp2Eq  | CaretEq | AmpEq   | PipeEq
                           | Star2Eq)
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
