use crate::field::End;
use crate::field::Entry;
use crate::field::Priority;
use crate::field::{Key, Pair, Value};
use crate::taskiter::TaskIter;
use crate::token::Token;
use std::ops::Range;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Task<'a>(&'a str);

#[derive(Clone, Debug)]
pub struct TaskBuf(String);

impl<'a> Task<'a> {
    pub fn new(str: &'a str) -> Self {
        Task(str)
    }

    pub fn iter(&self) -> TaskIter {
        TaskIter::new(&self.0)
    }

    pub fn into_iter(self) -> TaskIter<'a> {
        TaskIter::new(self.0)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_str(self) -> &'a str {
        self.0
    }

    pub fn is_completed(&self) -> bool {
        self.find_marker().is_some()
    }

    pub fn get_end(&self) -> Option<End> {
        match self.find_end() {
            Some((end, _)) => Some(end),
            _ => None,
        }
    }

    pub fn get_priority(&self) -> Option<Priority> {
        match self.find_priority() {
            Some((pri, _)) => Some(pri),
            _ => None,
        }
    }

    pub fn get_entry(&self) -> Option<Entry> {
        match self.find_entry() {
            Some((entry, _)) => Some(entry),
            _ => None,
        }
    }

    pub fn has_token(&self, token: &Token) -> bool {
        self.iter().any(|(t, _)| &t == token)
    }

    pub fn get_value(&self, key: &Key) -> Option<Value> {
        match self.find_pair(key) {
            Some((pair, _)) => Some(pair.value),
            _ => None,
        }
    }

    fn find_marker(&self) -> Option<Range<usize>> {
        self.iter().take(1).find_map(|(token, range)| match token {
            Token::Marker(_) => Some(range),
            _ => None,
        })
    }

    fn find_end(&self) -> Option<(End, Range<usize>)> {
        self.iter().take(3).find_map(|(token, range)| match token {
            Token::End(end) => Some((end, range)),
            _ => None,
        })
    }

    fn find_priority(&self) -> Option<(Priority, Range<usize>)> {
        self.iter().take(5).find_map(|(token, range)| match token {
            Token::Priority(pri) => Some((pri, range)),
            _ => None,
        })
    }

    fn find_entry(&self) -> Option<(Entry, Range<usize>)> {
        self.iter().take(7).find_map(|(token, range)| match token {
            Token::Entry(entry) => Some((entry, range)),
            _ => None,
        })
    }

    fn find_pair(&self, key: &Key) -> Option<(Pair, Range<usize>)> {
        for (token, range) in self.iter() {
            if let Token::Pair(pair) = token {
                if pair.key == *key {
                    return Some((pair, range));
                }
            }
        }
        None
    }

    fn find_body_start(&self) -> usize {
        self.iter()
            .find_map(|(token, range)| match token {
                Token::Marker(_)
                | Token::End(_)
                | Token::Priority(_)
                | Token::Entry(_)
                | Token::Space(_) => None,
                _ => Some(range.start),
            })
            .unwrap_or_else(|| self.0.len())
    }
}

impl TaskBuf {
    pub fn new(str: String) -> Self {
        TaskBuf(str)
    }

    pub fn as_task(&self) -> Task {
        Task::new(&self.0)
    }

    pub fn into_string(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn set_to(&mut self, task: &Task) {
        self.0.replace_range(.., task.as_str())
    }

    pub fn append_text(&mut self, text: &str) {
        if self.0.ends_with(|c: char| c.is_ascii_whitespace()) {
            if text.chars().all(|c: char| c.is_ascii_whitespace()) {
                return;
            }
        } else if !self.0.is_empty() && text.starts_with(|c: char| !c.is_ascii_whitespace()) {
            self.0.push(' ')
        }
        self.0.push_str(text);
    }

    pub fn clear_body(&mut self) {
        self.0.replace_range(self.as_task().find_body_start().., "");
    }

    pub fn remove_token(&mut self, token: &Token) {
        if let Some((_, range)) = self.as_task().iter().find(|(t, _)| t == token) {
            self.remove_range_with_whitespace(range);
        }
    }

    pub fn remove_pair(&mut self, key: &Key) {
        if let Some((_, range)) = self.as_task().find_pair(key) {
            self.remove_range_with_whitespace(range)
        }
    }

    pub fn set_value(&mut self, key: &Key, value: &Value) {
        match self.as_task().find_pair(key) {
            Some((_, range)) => self
                .0
                .replace_range(range.start + key.len() + 1..range.end, value.as_str()),
            None => {
                if self.0.ends_with(|c: char| !c.is_ascii_whitespace()) {
                    self.0.push(' ');
                }
                self.0.push_str(key.as_str());
                self.0.push(':');
                self.0.push_str(value.as_str())
            }
        }
    }

    pub fn set_pending(&mut self) {
        if let Some(range) = self.as_task().find_marker() {
            self.remove_range_with_whitespace(range)
        }
    }

    pub fn set_completed(&mut self) {
        match self {
            _ if self.as_task().is_completed() => {}
            _ if self.0.is_empty() => self.0.insert(0, 'x'),
            _ if self.0.starts_with(|c: char| c.is_ascii_whitespace()) => self.0.insert(0, 'x'),
            _ => self.0.insert_str(0, "x "),
        }
    }

    pub fn set_end(&mut self, end: Option<End>) {
        let existing_range = match self.as_task().find_end() {
            Some((_, range)) => Some(range),
            _ => None,
        };

        match (end, existing_range) {
            (Some(new_end), Some(existing_range)) => {
                self.0.replace_range(existing_range, new_end.as_str());
            }
            (Some(new_end), None) => {
                if let Some(range) = self.as_task().find_marker() {
                    self.0.insert_str(range.end, new_end.as_str());
                    self.0.insert(range.end, ' ');
                }
            }
            (None, Some(existing_range)) => {
                self.remove_range_with_whitespace(existing_range);
            }
            (None, None) => {}
        }
    }

    pub fn set_priority(&mut self, pri: Option<Priority>) {
        let existing_range = match self.as_task().find_priority() {
            Some((_, range)) => Some(range),
            _ => None,
        };

        match (pri, existing_range) {
            (Some(new_pri), Some(existing_range)) => {
                let buf: [u8; 3] = [b'(', new_pri.as_u8(), b')'];
                let new_pri = unsafe { std::str::from_utf8_unchecked(&buf) };
                self.0.replace_range(existing_range, new_pri);
            }
            (Some(new_pri), None) => {
                let buf: [u8; 3] = [b'(', new_pri.as_u8(), b')'];
                let new_pri = unsafe { std::str::from_utf8_unchecked(&buf) };
                if let Some((_, entry_range)) = self.as_task().find_entry() {
                    self.0.insert(entry_range.start, ' ');
                    self.0.insert_str(entry_range.start, new_pri);
                } else {
                    let body_start = self.as_task().find_body_start();
                    if self.0.len() != body_start {
                        self.0.insert(body_start, ' ');
                    }
                    self.0.insert_str(body_start, new_pri);
                }
            }
            (None, Some(existing_range)) => {
                self.remove_range_with_whitespace(existing_range);
            }
            (None, None) => {}
        }
    }

    pub fn set_entry(&mut self, entry: Option<Entry>) {
        let existing_range = match self.as_task().find_entry() {
            Some((_, range)) => Some(range),
            _ => None,
        };

        match (entry, existing_range) {
            (Some(new_entry), Some(existing_range)) => {
                self.0.replace_range(existing_range, new_entry.as_str());
            }
            (Some(new_entry), None) => {
                let body_start = self.as_task().find_body_start();
                if self.0.len() != body_start {
                    self.0.insert(body_start, ' ');
                }
                self.0.insert_str(body_start, new_entry.as_str());
            }
            (None, Some(existing_range)) => {
                self.remove_range_with_whitespace(existing_range);
            }
            (None, None) => {}
        }
    }

    fn remove_range_with_whitespace(&mut self, mut range: Range<usize>) {
        if let Some(trailing_space) = self
            .0
            .get(range.end..)
            .and_then(|str| str.split(|c: char| !c.is_ascii_whitespace()).next())
            .filter(|str| !str.is_empty())
        {
            range.end += trailing_space.len();
        } else if let Some(preceeding_space) = self
            .0
            .get(..range.start)
            .and_then(|str| str.rsplit(|c: char| !c.is_ascii_whitespace()).next())
            .filter(|str| !str.is_empty())
        {
            range.start -= preceeding_space.len();
        }
        self.0.replace_range(range, "");
    }
}
