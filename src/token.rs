// An enum over the possible crate::field values.

use crate::field::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Token<'a> {
    Annotation(Annotation),
    Context(Context<'a>),
    End(End<'a>),
    Entry(Entry<'a>),
    Key(Key<'a>),
    Marker(Marker),
    Normal(Normal<'a>),
    Pair(Pair<'a>),
    Priority(Priority),
    Project(Project<'a>),
    Space(Space<'a>),
}

impl<'a> Token<'a> {
    pub fn new(str: &'a str, stage: Stage) -> (Token<'a>, Stage) {
        if let Some(space) = Space::new(str) {
            match stage == Stage::Marker && !str.is_empty() {
                true => (Token::Space(space), Stage::End),
                false => (Token::Space(space), stage),
            }
        } else if let Some(marker) = Marker::new(str, stage) {
            (Token::Marker(marker), Stage::End)
        } else if let Some(end) = End::new(str, stage) {
            (Token::End(end), Stage::Priority)
        } else if let Some(pri) = Priority::new(str, stage) {
            (Token::Priority(pri), Stage::Entry)
        } else if let Some(entry) = Entry::new(str, stage) {
            (Token::Entry(entry), Stage::Body)
        } else if let Some(proj) = Project::new(str) {
            (Token::Project(proj), Stage::Body)
        } else if let Some(ctx) = Context::new(str) {
            (Token::Context(ctx), Stage::Body)
        } else if let Some(pair) = Pair::new(str) {
            (Token::Pair(pair), Stage::Body)
        } else if let Some(note) = Annotation::new(str) {
            (Token::Annotation(note), Stage::Body)
        } else {
            (Token::Normal(Normal::new(str)), Stage::Body)
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Token::Space(t) => t.len(),
            Token::Marker(t) => t.len(),
            Token::End(t) => t.len(),
            Token::Priority(t) => t.len(),
            Token::Entry(t) => t.len(),
            Token::Project(t) => t.len(),
            Token::Context(t) => t.len(),
            Token::Pair(t) => t.len(),
            Token::Key(t) => t.len(),
            Token::Annotation(t) => t.len(),
            Token::Normal(t) => t.len(),
        }
    }
}
