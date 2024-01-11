use crate::{diagnostics, string_builder::AutoCow, Kind, Lexer};

/// Lookup table mapping any incoming byte to a handler function defined below.
/// <https://github.com/ratel-rust/ratel-core/blob/master/ratel/src/lexer/mod.rs>
///
type ByteHandler = fn(&mut Lexer<'_>) -> Kind;

#[rustfmt::skip]
pub static BYTE_HANDLERS: [ByteHandler; 128] = [
//  0    1    2    3    4    5    6    7    8    9    A    B    C    D    E    F    //
    ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, SPS, ERR, SPS, SPS, ERR, ERR, ERR, // 0
    ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, // 1
    SPS, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, // 2
    ZER, DIG, DIG, DIG, DIG, DIG, DIG, DIG, DIG, DIG, ERR, SEM, ERR, EQL, ERR, ERR, // 3
    ERR, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, // 4
    IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, ERR, ERR, ERR, ERR, IDT, // 5
    ERR, ERR, ERR, L_C, ERR, ERR, ERR, ERR, IDT, ERR, IDT, ERR, L_L, ERR, ERR, ERR, // 6
    ERR, IDT, ERR, ERR, ERR, ERR, ERR, ERR, IDT, ERR, IDT, ERR, ERR, ERR, ERR, ERR, // 7
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

// =
const EQL: ByteHandler = |lexer| {
    lexer.consume_char();
    Kind::Eq
};

// ;
const SEM: ByteHandler = |lexer| {
    lexer.consume_char();
    Kind::Semicolon
};

const L_L: ByteHandler = |lexer| match &lexer.identifier_name_handler()[1..] {
    "et" => Kind::Let,
    _ => Kind::Ident,
};

const L_C: ByteHandler = |lexer| match &lexer.identifier_name_handler()[1..] {
    "onst" => Kind::Const,
    _ => Kind::Ident,
};
