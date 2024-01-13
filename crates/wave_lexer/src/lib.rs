mod byte_handlers;
mod diagnostics;
mod kind;
mod string_builder;
mod token;
mod trivia_builder;

use crate::string_builder::AutoCow;
use std::{collections::VecDeque, str::Chars};
use wave_allocator::Allocator;
use wave_diagnostics::Error;
use wave_span::Span;
use wave_syntax::{
    identifier::{
        is_identifier_part, is_identifier_start_all, is_irregular_line_terminator,
        is_irregular_whitespace, is_line_terminator,
    },
    unicode_id_start::is_id_start_unicode,
};

use self::trivia_builder::TriviaBuilder;
pub use self::{kind::Kind, token::Token};

#[derive(Debug, Clone)]
pub struct LexerCheckpoint<'a> {
    token: Token,
    errors_pos: usize,
    chars: Chars<'a>,
}

pub struct Lexer<'a> {
    allocator: &'a Allocator,
    source: &'a str,
    current: LexerCheckpoint<'a>,
    pub errors: Vec<Error>,
    lookahead: VecDeque<LexerCheckpoint<'a>>,
    pub trivia_builder: TriviaBuilder,
}

impl<'a> Lexer<'a> {
    pub fn new(allocator: &'a Allocator, source: &'a str) -> Self {
        let token = Token {
            is_on_new_line: true,
            ..Token::default()
        };

        let current = LexerCheckpoint {
            chars: source.chars(),
            token,
            errors_pos: 0,
        };

        Self {
            allocator,
            source,
            current,
            errors: vec![],
            lookahead: VecDeque::with_capacity(4),
            trivia_builder: TriviaBuilder::default(),
        }
    }

    pub fn remaining(&self) -> &'a str {
        self.current.chars.as_str()
    }

    pub fn checkpoint(&self) -> LexerCheckpoint<'a> {
        LexerCheckpoint {
            chars: self.current.chars.clone(),
            token: self.current.token,
            errors_pos: self.errors.len(),
        }
    }

    /// Rewinds the lexer to the same state as when the passed in `checkpoint` was created.
    pub fn rewind(&mut self, checkpoint: LexerCheckpoint<'a>) {
        self.errors.truncate(checkpoint.errors_pos);
        self.current = checkpoint;
        self.lookahead.clear();
    }

    pub fn lookahead(&mut self, n: u8) -> Token {
        let n = n as usize;
        debug_assert!(n > 0);

        if self.lookahead.len() > n - 1 {
            return self.lookahead[n - 1].token;
        }

        let checkpoint = self.checkpoint();

        if let Some(checkpoint) = self.lookahead.back() {
            self.current = checkpoint.clone();
        }

        // reset the current token for `read_next_token`,
        // otherwise it will contain the token from
        // `self.current = checkpoint`
        self.current.token = Token::default();

        for _i in self.lookahead.len()..n {
            let kind = self.read_next_token();
            let peeked = self.finish_next(kind);
            self.lookahead.push_back(LexerCheckpoint {
                chars: self.current.chars.clone(),
                token: peeked,
                errors_pos: self.errors.len(),
            });
        }

        self.current = checkpoint;

        self.lookahead[n - 1].token
    }

    /// Read each char and set the current token
    /// Whitespace and line terminators are skipped
    fn read_next_token(&mut self) -> Kind {
        loop {
            let offset = self.offset();
            self.current.token.start = offset;

            if let Some(c) = self.current.chars.clone().next() {
                let kind = self.match_char(c);
                if !matches!(
                    kind,
                    Kind::WhiteSpace | Kind::NewLine | Kind::Comment | Kind::MultiLineComment
                ) {
                    return kind;
                }
            } else {
                return Kind::Eof;
            }
        }
    }

    fn finish_next(&mut self, kind: Kind) -> Token {
        self.current.token.kind = kind;
        self.current.token.end = self.offset();
        debug_assert!(self.current.token.start <= self.current.token.end);
        let token = self.current.token;
        self.current.token = Token::default();
        token
    }

    #[inline]
    fn offset(&self) -> u32 {
        (self.source.len() - self.current.chars.as_str().len()) as u32
    }

    #[inline]
    fn consume_char(&mut self) -> char {
        self.current.chars.next().unwrap()
    }

    #[inline]
    fn next_eq(&mut self, c: char) -> bool {
        let matched = self.peek() == Some(c);
        if matched {
            self.current.chars.next();
        }
        matched
    }

    /// Peek the next char without advancing the position
    #[inline]
    fn peek(&self) -> Option<char> {
        self.current.chars.clone().next()
    }

    /// Section 12.7.1 Identifier Names
    fn identifier_tail(&mut self, mut builder: AutoCow<'a>) -> &'a str {
        // ident tail
        while let Some(c) = self.peek() {
            if !is_identifier_part(c) {
                if c == '\\' {
                    self.current.chars.next();
                    builder.force_allocation_without_current_ascii_char(self);
                    continue;
                }
                break;
            }
            self.current.chars.next();
            builder.push_matching(c);
        }
        let text = builder.finish(self);
        text
    }

    fn identifier_name(&mut self, builder: AutoCow<'a>) -> &'a str {
        self.identifier_tail(builder)
    }

    pub fn next_token(&mut self) -> Token {
        if let Some(checkpoint) = self.lookahead.pop_front() {
            self.current.chars = checkpoint.chars;
            self.current.errors_pos = checkpoint.errors_pos;
            return checkpoint.token;
        }
        let kind = self.read_next_token();
        self.finish_next(kind)
    }

    fn identifier_name_handler(&mut self) -> &'a str {
        let builder = AutoCow::new(self);
        self.consume_char();
        self.identifier_name(builder)
    }

    #[inline]
    fn match_char(&mut self, c: char) -> Kind {
        let size = c as usize;

        if size < 128 {
            return byte_handlers::BYTE_HANDLERS[size](self);
        }

        match c {
            c if is_id_start_unicode(c) => {
                let mut builder = AutoCow::new(self);
                let c = self.consume_char();
                builder.push_matching(c);
                self.identifier_name(builder);
                Kind::Ident
            }
            c if is_irregular_whitespace(c) => {
                self.trivia_builder
                    .add_irregular_whitespace(self.current.token.start, self.offset());
                self.consume_char();
                Kind::WhiteSpace
            }
            c if is_irregular_line_terminator(c) => {
                self.consume_char();
                self.current.token.is_on_new_line = true;
                Kind::NewLine
            }
            _ => {
                self.consume_char();
                self.error(diagnostics::InvalidCharacter(c, self.unterminated_range()));
                Kind::Undetermined
            }
        }
    }

    fn unterminated_range(&self) -> Span {
        Span::new(self.current.token.start, self.offset())
    }

    fn error<T: Into<Error>>(&mut self, error: T) {
        self.errors.push(error.into());
    }

    pub fn get_string(&self, token: Token) -> &'a str {
        &self.source[token.start as usize..token.end as usize]
    }

    fn skip_irregular_whitespace(&mut self) -> Kind {
        Kind::WhiteSpace
    }

    /// 12.9.3 Numeric Literals with `0` prefix
    fn read_zero(&mut self, _builder: &mut AutoCow<'a>) -> Kind {
        self.check_after_numeric_literal(Kind::Decimal)
    }

    fn check_after_numeric_literal(&mut self, kind: Kind) -> Kind {
        let offset = self.offset();
        // The SourceCharacter immediately following a NumericLiteral must not be an IdentifierStart or DecimalDigit.
        let c = self.peek();
        if c.is_none() || c.is_some_and(|ch| !ch.is_ascii_digit() && !is_identifier_start_all(ch)) {
            return kind;
        }
        self.current.chars.next();
        while let Some(c) = self.peek() {
            if is_identifier_start_all(c) {
                self.current.chars.next();
            } else {
                break;
            }
        }
        self.error(diagnostics::InvalidNumberEnd(Span::new(
            offset,
            self.offset(),
        )));
        Kind::Undetermined
    }

    fn read_decimal_digits_after_first_digit(&mut self, builder: &mut AutoCow<'a>) {
        while let Some(c) = self.peek() {
            match c {
                c @ '0'..='9' => {
                    self.current.chars.next();
                    builder.push_matching(c);
                }
                _ => break,
            }
        }
    }

    fn decimal_literal_after_first_digit(&mut self, builder: &mut AutoCow<'a>) -> Kind {
        self.read_decimal_digits_after_first_digit(builder);
        self.check_after_numeric_literal(Kind::Decimal)
    }

    fn read_string_literal(&mut self, delimiter: char) -> Kind {
        let mut builder = AutoCow::new(self);
        loop {
            match self.current.chars.next() {
                None | Some('\r' | '\n') => {
                    self.error(diagnostics::UnterminatedString(self.unterminated_range()));
                    return Kind::Undetermined;
                }
                Some(c @ ('"' | '\'')) => {
                    if c == delimiter {
                        builder.finish_without_push(self);
                        return Kind::Str;
                    }
                    builder.push_matching(c);
                }
                Some(c) => {
                    builder.push_matching(c);
                }
            }
        }
    }

    fn skip_single_line_comment(&mut self) -> Kind {
        let start = self.current.token.start;
        while let Some(c) = self.current.chars.next() {
            if is_line_terminator(c) {
                self.current.token.is_on_new_line = true;
                self.trivia_builder
                    .add_single_line_comment(start, self.offset() - c.len_utf8() as u32);
                return Kind::Comment;
            }
        }
        // EOF
        self.trivia_builder
            .add_single_line_comment(start, self.offset());
        Kind::Comment
    }

    fn skip_multi_line_comment(&mut self) -> Kind {
        while let Some(c) = self.current.chars.next() {
            if c == '*' && self.next_eq('/') {
                self.trivia_builder
                    .add_multi_line_comment(self.current.token.start, self.offset());
                return Kind::MultiLineComment;
            }
            if is_line_terminator(c) {
                self.current.token.is_on_new_line = true;
            }
        }
        self.error(diagnostics::UnterminatedMultiLineComment(
            self.unterminated_range(),
        ));
        Kind::Eof
    }
}
