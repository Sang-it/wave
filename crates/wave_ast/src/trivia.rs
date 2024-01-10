use std::collections::BTreeMap;
use wave_span::Span;

#[derive(Debug, Default)]
pub struct Trivias {
    pub comments: Vec<(u32, u32, CommentKind)>,
    pub irregular_whitespaces: Vec<Span>,
}

#[derive(Debug, Default)]
pub struct TriviasMap {
    comments: BTreeMap<u32, Comment>,
    irregular_whitespaces: Vec<Span>,
}

impl From<Trivias> for TriviasMap {
    fn from(trivias: Trivias) -> Self {
        Self {
            comments: trivias
                .comments
                .iter()
                .map(|t| (t.0, Comment::new(t.1, t.2)))
                .collect(),
            irregular_whitespaces: trivias.irregular_whitespaces,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(unused)]
pub struct Comment {
    kind: CommentKind,
    end: u32,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CommentKind {
    SingleLine,
    MultiLine,
}

impl CommentKind {
    pub fn is_single_line(self) -> bool {
        matches!(self, Self::SingleLine)
    }

    pub fn is_multi_line(self) -> bool {
        matches!(self, Self::MultiLine)
    }
}

impl Comment {
    pub fn new(end: u32, kind: CommentKind) -> Self {
        Self { kind, end }
    }

    pub fn end(self) -> u32 {
        self.end
    }

    pub fn kind(&self) -> CommentKind {
        self.kind
    }

    pub fn is_single_line(self) -> bool {
        self.kind.is_single_line()
    }

    pub fn is_multi_line(self) -> bool {
        self.kind.is_multi_line()
    }
}

impl TriviasMap {
    pub fn comments(&self) -> &BTreeMap<u32, Comment> {
        &self.comments
    }

    pub fn has_comments_between(&self, span: Span) -> bool {
        self.comments.range(span.start..span.end).count() > 0
    }

    pub fn add_single_line_comment(&mut self, span: Span) {
        let comment = Comment::new(span.end, CommentKind::SingleLine);
        self.comments.insert(span.start, comment);
    }

    pub fn add_multi_line_comment(&mut self, span: Span) {
        let comment = Comment::new(span.end, CommentKind::MultiLine);
        self.comments.insert(span.start, comment);
    }

    pub fn comments_spans(&self) -> impl Iterator<Item = (Comment, Span)> + '_ {
        self.comments()
            .iter()
            .map(|(start, comment)| (*comment, Span::new(*start, comment.end)))
    }

    pub fn irregular_whitespaces(&self) -> &Vec<Span> {
        &self.irregular_whitespaces
    }
}
