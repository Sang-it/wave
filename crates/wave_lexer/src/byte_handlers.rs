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
    SPS, IDT, QOT, IDT, IDT, IDT, IDT, QOT, PNO, PNC, ATR, PLS, COM, MIN, IDT, SLH, // 2
    ZER, DIG, DIG, DIG, DIG, DIG, DIG, DIG, DIG, DIG, IDT, SEM, IDT, EQL, IDT, IDT, // 3
    IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, // 4
    IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, // 5
    IDT, IDT, IDT, L_C, IDT, L_E, L_F, IDT, IDT, L_I, IDT, IDT, L_L, IDT, IDT, IDT, // 6
    IDT, IDT, L_R, IDT, L_T, IDT, IDT, IDT, IDT, IDT, IDT, BEO, IDT, BEC, IDT, ERR, // 7
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
    Kind::Plus
};

// -
const MIN: ByteHandler = |lexer| {
    lexer.consume_char();
    Kind::Minus
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
        _ => Kind::Slash,
    }
};

// *
const ATR: ByteHandler = |lexer| {
    lexer.consume_char();
    if lexer.next_eq('*') {
        Kind::Star2
    } else {
        Kind::Star
    }
};

const COM: ByteHandler = |lexer| {
    lexer.consume_char();
    Kind::Comma
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

const L_C: ByteHandler = |lexer| match &lexer.identifier_name_handler()[1..] {
    "onst" => Kind::Const,
    _ => Kind::Ident,
};

const L_E: ByteHandler = |lexer| match &lexer.identifier_name_handler()[1..] {
    "lse" => Kind::Else,
    _ => Kind::Ident,
};

const L_F: ByteHandler = |lexer| match &lexer.identifier_name_handler()[1..] {
    "unction" => Kind::Function,
    "alse" => Kind::False,
    _ => Kind::Ident,
};

const L_I: ByteHandler = |lexer| match &lexer.identifier_name_handler()[1..] {
    "f" => Kind::If,
    _ => Kind::Ident,
};

const L_L: ByteHandler = |lexer| match &lexer.identifier_name_handler()[1..] {
    "et" => Kind::Let,
    _ => Kind::Ident,
};

const L_T: ByteHandler = |lexer| match &lexer.identifier_name_handler()[1..] {
    "rue" => Kind::True,
    _ => Kind::Ident,
};

const L_R: ByteHandler = |lexer| match &lexer.identifier_name_handler()[1..] {
    "eturn" => Kind::Return,
    _ => Kind::Ident,
};
