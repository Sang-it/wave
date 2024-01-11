use crate::Parser;
use wave_lexer::{Kind, LexerCheckpoint, Token};
use wave_span::Span;

pub struct ParserCheckpoint<'a> {
    lexer: LexerCheckpoint<'a>,
    cur_token: Token,
    prev_span_end: u32,
    errors_pos: usize,
}

impl<'a> Parser<'a> {
    pub(crate) fn start_span(&self) -> Span {
        let token = self.cur_token();
        Span::new(token.start, 0)
    }

    pub(crate) fn end_span(&self, span: Span) -> Span {
        let mut span = span;
        span.end = self.prev_token_end;
        span
    }

    /// Get current source text
    pub(crate) fn cur_src(&self) -> &'a str {
        let range = self.cur_token().span();
        // SAFETY:
        // range comes from the parser, which are ensured to meeting the criteria of `get_unchecked`.
        unsafe {
            self.source_text
                .get_unchecked(range.start as usize..range.end as usize)
        }
    }

    /// Advance and return true if we are at `Kind`, return false otherwise
    pub(crate) fn eat(&mut self, kind: Kind) -> bool {
        if self.at(kind) {
            self.advance(kind);
            return true;
        }
        false
    }

    /// Get current string
    pub(crate) fn cur_string(&self) -> &'a str {
        self.lexer.get_string(self.token)
    }

    /// Peek next kind, returns EOF for final peek
    pub(crate) fn peek_kind(&mut self) -> Kind {
        self.peek_token().kind
    }

    /// Peek next token, returns EOF for final peek
    pub(crate) fn peek_token(&mut self) -> Token {
        self.lexer.lookahead(1)
    }

    /// Get current token
    pub(crate) fn cur_token(&self) -> Token {
        self.token
    }

    /// Get current Kind
    pub(crate) fn cur_kind(&self) -> Kind {
        self.token.kind
    }

    pub(crate) fn at(&self, kind: Kind) -> bool {
        self.cur_kind() == kind
    }

    /// Move to the next token
    /// Checks if the current token is escaped if it is a keyword
    fn advance(&mut self, _kind: Kind) {
        self.prev_token_end = self.token.end;
        self.token = self.lexer.next_token();
    }

    /// Advance any token
    pub(crate) fn bump_any(&mut self) {
        self.advance(self.cur_kind());
    }

    /// Advance and change token type, useful for changing keyword to ident
    pub(crate) fn bump_remap(&mut self, kind: Kind) {
        self.advance(kind);
    }
}
