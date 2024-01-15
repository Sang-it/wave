use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Context: u8 {
        const In = 1<< 0;
        const Return = 1<< 1;
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::In
    }
}

impl Context {
    #[inline]
    pub(crate) fn has_return(self) -> bool {
        self.contains(Self::Return)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum StatementContext {
    If,
    While,
    StatementList,
}

impl StatementContext {
    pub(crate) fn is_single_statement(self) -> bool {
        self != Self::StatementList
    }
}
