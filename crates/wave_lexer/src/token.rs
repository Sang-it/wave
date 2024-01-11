use super::kind::Kind;
use wave_span::Span;

#[derive(Debug, Clone, Copy, Default)]
pub struct Token {
    pub kind: Kind,
    pub start: u32,
    pub end: u32,
    pub is_on_new_line: bool,
}

impl Token {
    pub fn span(&self) -> Span {
        Span::new(self.start, self.end)
    }
}
