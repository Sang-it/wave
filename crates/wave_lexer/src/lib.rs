mod diagnostics;
mod kind;
mod number;
mod string_builder;
mod token;
mod trivia_builder;

use crate::string_builder::AutoCow;
use std::{collections::VecDeque, str::Chars};
use wave_allocator::Allocator;
use wave_diagnostics::Error;
use wave_span::Span;
use wave_syntax::{
    identifier::{is_identifier_part, is_irregular_line_terminator, is_irregular_whitespace},
    unicode_id_start::is_id_start_unicode,
};

use self::trivia_builder::TriviaBuilder;
pub use self::{kind::Kind, number::parse_int, token::Token};

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
    pub(crate) errors: Vec<Error>,
    lookahead: VecDeque<LexerCheckpoint<'a>>,
    pub(crate) trivia_builder: TriviaBuilder,
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

    fn identifier_name_handler(&mut self) -> &'a str {
        let builder = AutoCow::new(self);
        self.consume_char();
        self.identifier_name(builder)
    }

    #[inline]
    fn match_char(&mut self, c: char) -> Kind {
        let _size = c as usize;

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
}
