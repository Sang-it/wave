use wave_allocator::String;

use crate::Lexer;

pub struct AutoCow<'a> {
    pub start: &'a str,
    pub value: Option<String<'a>>,
}

impl<'a> AutoCow<'a> {
    pub fn new(lexer: &Lexer<'a>) -> Self {
        let start = lexer.remaining();
        AutoCow { start, value: None }
    }

    pub fn push_matching(&mut self, c: char) {
        if let Some(text) = &mut self.value {
            text.push(c);
        }
    }

    // Force allocation of a String, excluding the current ASCII character.
    pub fn force_allocation_without_current_ascii_char(&mut self, lexer: &Lexer<'a>) {
        if self.value.is_some() {
            return;
        }
        self.value = Some(String::from_str_in(
            &self.start[..self.start.len() - lexer.remaining().len() - 1],
            lexer.allocator,
        ));
    }

    pub fn finish(mut self, lexer: &Lexer<'a>) -> &'a str {
        match self.value.take() {
            Some(s) => s.into_bump_str(),
            None => &self.start[..self.start.len() - lexer.remaining().len()],
        }
    }

    // Just like finish, but without pushing current char.
    pub fn finish_without_push(mut self, lexer: &Lexer<'a>) -> &'a str {
        match self.value.take() {
            Some(s) => s.into_bump_str(),
            None => &self.start[..self.start.len() - lexer.remaining().len() - 1],
        }
    }
}
