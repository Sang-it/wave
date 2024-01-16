use crate::{context::Context, diagnostics, Parser};
use wave_diagnostics::Result;
use wave_lexer::{Kind, LexerCheckpoint, Token};
use wave_span::Span;

#[allow(dead_code)]
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

    /// Peek next token, returns EOF for final peek
    pub(crate) fn peek_token(&mut self) -> Token {
        self.lexer.lookahead(1)
    }

    /// Peek at kind
    pub(crate) fn peek_at(&mut self, kind: Kind) -> bool {
        self.peek_token().kind == kind
    }

    /// Get current token
    pub(crate) fn cur_token(&self) -> Token {
        self.token
    }

    /// Get current Kind
    pub(crate) fn cur_kind(&self) -> Kind {
        self.token.kind
    }

    /// Peek next kind, returns EOF for final peek
    pub(crate) fn peek_kind(&mut self) -> Kind {
        self.peek_token().kind
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

    pub(crate) fn asi(&mut self) -> Result<()> {
        if !self.can_insert_semicolon() {
            let span = Span::new(self.prev_token_end, self.cur_token().start);
            return Err(diagnostics::AutoSemicolonInsertion(span).into());
        }
        if self.at(Kind::Semicolon) {
            self.advance(Kind::Semicolon);
        }
        Ok(())
    }

    pub(crate) fn expect(&mut self, kind: Kind) -> Result<()> {
        self.expect_without_advance(kind)?;
        self.advance(kind);
        Ok(())
    }

    pub(crate) fn expect_without_advance(&mut self, kind: Kind) -> Result<()> {
        if !self.at(kind) {
            let range = self.cur_token().span();
            return Err(
                diagnostics::ExpectToken(kind.to_str(), self.cur_kind().to_str(), range).into(),
            );
        }
        Ok(())
    }

    pub(crate) fn can_insert_semicolon(&self) -> bool {
        let kind = self.cur_kind();
        if kind == Kind::Semicolon {
            return true;
        }
        kind == Kind::RCurly || kind.is_eof() || self.cur_token().is_on_new_line
    }

    pub(crate) fn with_context<F, T>(&mut self, flags: Context, cb: F) -> T
    where
        F: FnOnce(&mut Self) -> T,
    {
        let context_flags_to_set = flags & !self.ctx;
        if !context_flags_to_set.is_empty() {
            self.ctx |= context_flags_to_set;
            let result = cb(self);
            self.ctx &= !context_flags_to_set;
            return result;
        }
        cb(self)
    }
}
