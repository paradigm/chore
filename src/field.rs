// Underlying typed representation of task fields, shared across other parts of the code base

use std::ops::RangeInclusive;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Annotation;
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Context<'a>(&'a str);
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct End<'a>(pub &'a str);
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Entry<'a>(pub &'a str);
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Key<'a>(&'a str);
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Marker;
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Normal<'a>(&'a str);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Number(usize);
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pair<'a> {
    pub key: Key<'a>,
    pub value: Value<'a>,
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Priority(u8);
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Project<'a>(&'a str);
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Space<'a>(&'a str);
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Value<'a>(&'a str);

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum Stage {
    Marker,
    End,
    Priority,
    Entry,
    Body,
}

impl Annotation {
    pub fn new(str: &str) -> Option<Self> {
        match str == "|" {
            true => Some(Annotation),
            false => None,
        }
    }

    pub fn len(&self) -> usize {
        "|".len()
    }
}

impl<'a> Context<'a> {
    pub fn new(str: &'a str) -> Option<Self> {
        match str.starts_with('@') {
            true => Some(Context(str)),
            false => None,
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn as_str(&self) -> &str {
        self.0
    }
}

impl<'a> End<'a> {
    pub fn new(str: &'a str, stage: Stage) -> Option<Self> {
        match stage == Stage::End && is_yyyy_mm_dd(str) {
            true => Some(End(str)),
            false => None,
        }
    }

    pub fn as_str(&self) -> &'a str {
        self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a> Entry<'a> {
    pub fn new(str: &'a str, stage: Stage) -> Option<Self> {
        match stage <= Stage::Entry && is_yyyy_mm_dd(str) {
            true => Some(Entry(str)),
            false => None,
        }
    }

    pub fn as_str(&self) -> &'a str {
        self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a> Key<'a> {
    pub fn new(str: &'a str) -> Self {
        Key(str)
    }

    pub fn as_str(&self) -> &'a str {
        self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Marker {
    pub fn new(str: &str, stage: Stage) -> Option<Self> {
        match stage == Stage::Marker && str == "x" {
            true => Some(Marker),
            false => None,
        }
    }

    pub fn len(&self) -> usize {
        "x".len()
    }
}

impl<'a> Normal<'a> {
    pub fn new(str: &'a str) -> Self {
        Normal(str)
    }

    pub fn as_str(&self) -> &'a str {
        self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Number {
    pub fn from_enumerate(nr: usize) -> Self {
        // enumerate is zero indexed, line numbers are one indexed
        Number(nr + 1)
    }

    pub const fn from_usize(nr: usize) -> Self {
        Number(nr)
    }

    pub fn from_str(str: &str) -> Option<Self> {
        Some(Number(str.parse::<usize>().ok()?))
    }

    pub fn new_range(str: &str) -> Option<RangeInclusive<Self>> {
        let (a, b) = str.split_once('-')?;
        let a = Number::from_str(a)?;
        let b = Number::from_str(b)?;
        match a <= b {
            true => Some(a..=b),
            false => Some(b..=a),
        }
    }

    pub fn new_list(str: &str) -> Option<Vec<Self>> {
        let mut nrs = Vec::new();
        for str in str.split(',') {
            nrs.push(Number::from_str(str)?);
        }
        Some(nrs)
    }

    pub const fn digits(&self) -> usize {
        let mut nr = self.0;
        let mut len = 0;
        while nr > 0 {
            nr /= 10;
            len += 1;
        }
        len
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

impl<'a> Pair<'a> {
    pub fn new(str: &'a str) -> Option<Self> {
        str.split_once(':').map(|(k, v)| Pair {
            key: Key(k),
            value: Value(v),
        })
    }

    pub fn len(&self) -> usize {
        self.key.len() + ":".len() + self.value.len()
    }
}

impl Priority {
    pub fn new(str: &str, stage: Stage) -> Option<Self> {
        if stage > Stage::Priority {
            return None;
        }
        str.strip_prefix('(')
            .and_then(|str| str.strip_suffix(')'))
            .filter(|str| str.len() == 1)
            .and_then(|str| str.chars().next())
            .and_then(Priority::from_char)
    }

    pub fn new_range(str: &str) -> Option<RangeInclusive<Self>> {
        let mut chars = Some(str)
            .filter(|str| str.len() == 5)
            .and_then(|str| str.strip_prefix('('))
            .and_then(|str| str.strip_suffix(')'))
            .map(|str| str.chars())?;

        let a = Priority::from_char(chars.next()?)?;
        let _ = chars.next().filter(|&c| c == '-')?;
        let b = Priority::from_char(chars.next()?)?;
        match a <= b {
            true => Some(a..=b),
            false => Some(b..=a),
        }
    }

    pub fn from_char(c: char) -> Option<Self> {
        match ('A'..='Z').contains(&c) {
            true => Some(Priority(c as u8)),
            false => None,
        }
    }

    pub fn as_u8(&self) -> u8 {
        self.0 as u8
    }

    pub fn len(&self) -> usize {
        "(x)".len()
    }
}

impl<'a> Project<'a> {
    pub fn new(str: &'a str) -> Option<Self> {
        match str.starts_with('+') {
            true => Some(Project(str)),
            false => None,
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn as_str(&self) -> &str {
        self.0
    }
}

impl<'a> Space<'a> {
    pub fn new(str: &'a str) -> Option<Self> {
        match str.chars().all(|c| c.is_ascii_whitespace()) {
            true => Some(Space(str)),
            false => None,
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn as_str(&self) -> &str {
        self.0
    }
}

impl Stage {
    pub fn new() -> Stage {
        Stage::Marker
    }
}

impl<'a> Value<'a> {
    pub fn new(str: &'a str) -> Self {
        Value(str)
    }

    pub fn as_str(&self) -> &'a str {
        self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

fn is_yyyy_mm_dd(str: &str) -> bool {
    let mut chars = str.chars();
    chars.next().map(|c| c.is_ascii_digit()) == Some(true)
        && chars.next().map(|c| c.is_ascii_digit()) == Some(true)
        && chars.next().map(|c| c.is_ascii_digit()) == Some(true)
        && chars.next().map(|c| c.is_ascii_digit()) == Some(true)
        && chars.next().map(|c| c == '-') == Some(true)
        && chars.next().map(|c| c == '0' || c == '1') == Some(true)
        && chars.next().map(|c| c.is_ascii_digit()) == Some(true)
        && chars.next().map(|c| c == '-') == Some(true)
        && chars.next().map(|c| ('0'..='3').contains(&c)) == Some(true)
        && chars.next().map(|c| c.is_ascii_digit()) == Some(true)
        && chars.next().is_none()
}
