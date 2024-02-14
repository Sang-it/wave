use crate::{diagnostics, string_builder::AutoCow, Kind, Lexer};

/// Lookup table mapping any incoming byte to a handler function defined below.
/// <https://github.com/ratel-rust/ratel-core/blob/master/ratel/src/lexer/mod.rs>
///
type ByteHandler = fn(&mut Lexer<'_>) -> Kind;

#[rustfmt::skip]
pub static BYTE_HANDLERS: [ByteHandler; 128] = [
//  0    1    2    3    4    5    6    7    8    9    A    B    C    D    E    F    //
    ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, SPS, LIN, SPS, SPS, LIN, ERR, ERR, // 0
    ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, // 1
    SPS, EXL, QOT, IDT, IDT, PRC, AMP, QOT, PNO, PNC, ATR, PLS, COM, MIN, PRD, SLH, // 2
    ZER, DIG, DIG, DIG, DIG, DIG, DIG, DIG, DIG, DIG, IDT, SEM, LSS, EQL, GTR, IDT, // 3
    IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, // 4
    IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, BTO, IDT, BTC, CRT, IDT, // 5
    IDT, IDT, L_B, L_C, IDT, L_E, L_F, IDT, IDT, L_I, IDT, IDT, L_L, IDT, L_N, IDT, // 6
    IDT, IDT, L_R, L_S, L_T, IDT, IDT, L_W, IDT, IDT, IDT, BEO, PIP, BEC, IDT, ERR, // 7
];

const ERR: ByteHandler = |lexer| {
    let c = lexer.consume_char();
    lexer.error(diagnostics::InvalidCharacter(c, lexer.unterminated_range()));
    Kind::Undetermined
};

// <TAB> <VT> <FF>
const SPS: ByteHandler = |lexer| {
    lexer.skip_irregular_whitespace();
    lexer.consume_char();
    Kind::WhiteSpace
};

// '\r' '\n'
const LIN: ByteHandler = |lexer| {
    lexer.consume_char();
    lexer.current.token.is_on_new_line = true;
    Kind::NewLine
};

const IDT: ByteHandler = |lexer| {
    lexer.identifier_name_handler();
    Kind::Ident
};

// 0
const ZER: ByteHandler = |lexer| {
    let mut builder = AutoCow::new(lexer);
    let c = lexer.consume_char();
    builder.push_matching(c);
    lexer.read_zero(&mut builder)
};

// 1 to 9
const DIG: ByteHandler = |lexer| {
    let mut builder = AutoCow::new(lexer);
    let c = lexer.consume_char();
    builder.push_matching(c);
    lexer.decimal_literal_after_first_digit(&mut builder)
};

// <
const LSS: ByteHandler = |lexer| {
    lexer.consume_char();
    if lexer.next_eq('=') {
        Kind::LtEq
    } else {
        Kind::LAngle
    }
};

// >
const GTR: ByteHandler = |lexer| {
    lexer.consume_char();
    if lexer.next_eq('=') {
        Kind::GtEq
    } else {
        Kind::RAngle
    }
};

// %
const PRC: ByteHandler = |lexer| {
    lexer.consume_char();
    lexer.consume_char();
    if lexer.next_eq('=') {
        Kind::PercentEq
    } else {
        Kind::Percent
    }
};

// = ==
const EQL: ByteHandler = |lexer| {
    lexer.consume_char();
    if lexer.next_eq('=') {
        Kind::Eq2
    } else {
        Kind::Eq
    }
};

// ;
const SEM: ByteHandler = |lexer| {
    lexer.consume_char();
    Kind::Semicolon
};

// ' "
const QOT: ByteHandler = |lexer| {
    let c = lexer.consume_char();
    lexer.read_string_literal(c)
};

// +
const PLS: ByteHandler = |lexer| {
    lexer.consume_char();
    if lexer.next_eq('=') {
        Kind::PlusEq
    } else if lexer.next_eq('+') {
        Kind::Plus2
    } else {
        Kind::Plus
    }
};

// -
const MIN: ByteHandler = |lexer| {
    lexer.consume_char();
    if lexer.next_eq('=') {
        Kind::MinusEq
    } else if lexer.next_eq('-') {
        Kind::Minus2
    } else {
        Kind::Minus
    }
};

// /
const SLH: ByteHandler = |lexer| {
    lexer.consume_char();
    match lexer.peek() {
        Some('/') => {
            lexer.current.chars.next();
            lexer.skip_single_line_comment()
        }
        Some('*') => {
            lexer.current.chars.next();
            lexer.skip_multi_line_comment()
        }
        _ => {
            if lexer.next_eq('=') {
                Kind::SlashEq
            } else {
                Kind::Slash
            }
        }
    }
};

// *
const ATR: ByteHandler = |lexer| {
    lexer.consume_char();
    if lexer.next_eq('*') {
        Kind::Star2
    } else if lexer.next_eq('=') {
        Kind::StarEq
    } else {
        Kind::Star
    }
};

// &
const AMP: ByteHandler = |lexer| {
    lexer.consume_char();
    if lexer.next_eq('&') {
        if lexer.next_eq('=') {
            Kind::Amp2Eq
        } else {
            Kind::Amp2
        }
    } else if lexer.next_eq('=') {
        Kind::AmpEq
    } else {
        Kind::Amp
    }
};

// |
const PIP: ByteHandler = |lexer| {
    lexer.consume_char();
    if lexer.next_eq('|') {
        if lexer.next_eq('=') {
            Kind::Pipe2Eq
        } else {
            Kind::Pipe2
        }
    } else if lexer.next_eq('=') {
        Kind::PipeEq
    } else {
        Kind::Pipe
    }
};

// ^
const CRT: ByteHandler = |lexer| {
    lexer.consume_char();
    if lexer.next_eq('=') {
        Kind::CaretEq
    } else {
        Kind::Caret
    }
};

// ,
const COM: ByteHandler = |lexer| {
    lexer.consume_char();
    Kind::Comma
};

// [
const BTO: ByteHandler = |lexer| {
    lexer.consume_char();
    Kind::LBrack
};

// ]
const BTC: ByteHandler = |lexer| {
    lexer.consume_char();
    Kind::RBrack
};

// (
const PNO: ByteHandler = |lexer| {
    lexer.consume_char();
    Kind::LParen
};

// )
const PNC: ByteHandler = |lexer| {
    lexer.consume_char();
    Kind::RParen
};

// {
const BEO: ByteHandler = |lexer| {
    lexer.consume_char();
    Kind::LCurly
};

// }
const BEC: ByteHandler = |lexer| {
    lexer.consume_char();
    Kind::RCurly
};

// !
const EXL: ByteHandler = |lexer| {
    lexer.consume_char();
    if lexer.next_eq('=') {
        Kind::Neq
    } else {
        Kind::Bang
    }
};

// .
const PRD: ByteHandler = |lexer| {
    lexer.consume_char();
    Kind::Dot
};

const L_B: ByteHandler = |lexer| match &lexer.identifier_name_handler()[1..] {
    "reak" => Kind::Break,
    _ => Kind::Ident,
};

const L_C: ByteHandler = |lexer| match &lexer.identifier_name_handler()[1..] {
    "onst" => Kind::Const,
    "lass" => Kind::Class,
    "ontinue" => Kind::Continue,
    _ => Kind::Ident,
};

const L_E: ByteHandler = |lexer| match &lexer.identifier_name_handler()[1..] {
    "lse" => Kind::Else,
    "xtends" => Kind::Extends,
    _ => Kind::Ident,
};

const L_F: ByteHandler = |lexer| match &lexer.identifier_name_handler()[1..] {
    "unction" => Kind::Function,
    "alse" => Kind::False,
    _ => Kind::Ident,
};

const L_I: ByteHandler = |lexer| match &lexer.identifier_name_handler()[1..] {
    "f" => Kind::If,
    "mport" => Kind::Import,
    _ => Kind::Ident,
};

const L_L: ByteHandler = |lexer| match &lexer.identifier_name_handler()[1..] {
    "et" => Kind::Let,
    _ => Kind::Ident,
};

const L_N: ByteHandler = |lexer| match &lexer.identifier_name_handler()[1..] {
    "ull" => Kind::Null,
    "ew" => Kind::New,
    _ => Kind::Ident,
};

const L_T: ByteHandler = |lexer| match &lexer.identifier_name_handler()[1..] {
    "his" => Kind::This,
    "rue" => Kind::True,
    _ => Kind::Ident,
};

const L_R: ByteHandler = |lexer| match &lexer.identifier_name_handler()[1..] {
    "eturn" => Kind::Return,
    _ => Kind::Ident,
};

const L_S: ByteHandler = |lexer| match &lexer.identifier_name_handler()[1..] {
    "uper" => Kind::Super,
    _ => Kind::Ident,
};

const L_W: ByteHandler = |lexer| match &lexer.identifier_name_handler()[1..] {
    "hile" => Kind::While,
    _ => Kind::Ident,
};
