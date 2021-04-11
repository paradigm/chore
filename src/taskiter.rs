use crate::field::Stage;
use crate::token::Token;
use std::ops::Range;

pub struct TaskIter<'a> {
    remaining: &'a str,
    tokens: std::str::SplitAsciiWhitespace<'a>,
    offset: usize,
    stage: Stage,
}

impl<'a> TaskIter<'a> {
    pub fn new(str: &'a str) -> Self {
        TaskIter {
            remaining: str,
            tokens: str.split_ascii_whitespace(),
            offset: 0,
            stage: Stage::new(),
        }
    }
}

impl<'a> Iterator for TaskIter<'a> {
    type Item = (Token<'a>, Range<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        let token = if self
            .remaining
            .starts_with(|c: char| c.is_ascii_whitespace())
        {
            self.remaining
                .split(|c: char| !c.is_ascii_whitespace())
                .next()
        } else {
            self.tokens.next()
        }?;

        let (token, stage) = Token::new(token, self.stage);
        let len = token.len();
        let range = self.offset..self.offset + len;
        self.remaining = &self.remaining[len..];
        self.stage = stage;
        self.offset += len;

        Some((token, range))
    }
}
