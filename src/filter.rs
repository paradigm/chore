use crate::date::Date;
use crate::error::*;
use crate::field::*;
use crate::regex::Regex;
use crate::task::Task;
use crate::token::Token;
use std::ops::RangeInclusive;

pub struct Filter<'a> {
    kind: Kind<'a>,
    negate: bool,
}

enum Kind<'a> {
    All,
    IsCompleted,
    HasProject(Project<'a>),
    HasContext(Context<'a>),
    HasPair(FilterPair<'a>),
    MatchesRegex(Box<Regex>),
    WithinPriorityRange(RangeInclusive<Priority>),
    WithinNumberRange(RangeInclusive<Number>),
    WithinNumberList(Vec<Number>),
}

enum FilterPair<'a> {
    End(String),
    Priority(Priority),
    Entry(String),
    KeyValue(Key<'a>, String), // owned string for relative to absolute date conversion
    AnyEnd,
    AnyPriority,
    AnyEntry,
    AnyValue(Key<'a>),
    NoEnd,
    NoPriority,
    NoEntry,
    NoKey(Key<'a>),
    EndBefore(Date),
    EntryBefore(Date),
    ValueBefore(Key<'a>, Date),
    EndAfter(Date),
    EntryAfter(Date),
    ValueAfter(Key<'a>, Date),
    EndIn(Date),
    EntryIn(Date),
    ValueIn(Key<'a>, Date),
}

impl<'a> Filter<'a> {
    const ALL: &'static str = "all";
    const DONE: &'static str = "+done";

    pub fn new(str: &'a str, now: &Date, date_keys: &[Key]) -> Result<Option<Self>> {
        let (str, negate) = match str.strip_prefix('-').or_else(|| str.strip_prefix('!')) {
            Some(stripped_str) => (stripped_str, true),
            None => (str, false),
        };

        let kind = if str == Filter::ALL {
            Kind::All
        } else if str == Filter::DONE {
            Kind::IsCompleted
        } else if let Some(proj) = Project::new(str) {
            Kind::HasProject(proj)
        } else if let Some(ctx) = Context::new(str) {
            Kind::HasContext(ctx)
        } else if let Some(regex) = Regex::new(str)? {
            Kind::MatchesRegex(Box::new(regex))
        } else if let Some(pri) = Priority::new(str, Stage::Priority) {
            Kind::WithinPriorityRange(pri..=pri)
        } else if let Some(pri_range) = Priority::new_range(str) {
            Kind::WithinPriorityRange(pri_range)
        } else if let Some(nr_list) = Number::new_list(str) {
            Kind::WithinNumberList(nr_list)
        } else if let Some(nr_range) = Number::new_range(str) {
            Kind::WithinNumberRange(nr_range)
        } else if let Some(pair) = FilterPair::new(str, now, date_keys)? {
            Kind::HasPair(pair)
        } else {
            return Ok(None);
        };

        Ok(Some(Filter { kind, negate }))
    }

    pub fn keep(&self, task: &Task, nr: Number) -> bool {
        let result = match &self.kind {
            Kind::All => true,
            Kind::IsCompleted => task.is_completed(),
            Kind::HasProject(proj) => task.has_token(&Token::Project(proj.clone())),
            Kind::HasContext(ctx) => task.has_token(&Token::Context(ctx.clone())),
            Kind::HasPair(pair) => match pair {
                FilterPair::End(v) => task.get_end() == Some(End(v)),
                FilterPair::Priority(v) => task.get_priority() == Some(*v),
                FilterPair::Entry(v) => task.get_entry() == Some(Entry(v)),
                FilterPair::KeyValue(k, v) => task.get_value(k) == Some(Value::new(v)),
                FilterPair::AnyEnd => task.get_end().is_some(),
                FilterPair::AnyPriority => task.get_priority().is_some(),
                FilterPair::AnyEntry => task.get_entry().is_some(),
                FilterPair::AnyValue(k) => task.get_value(k).is_some(),
                FilterPair::NoEnd => task.get_end().is_none(),
                FilterPair::NoPriority => task.get_priority().is_none(),
                FilterPair::NoEntry => task.get_entry().is_none(),
                FilterPair::NoKey(k) => task.get_value(k).is_none(),
                FilterPair::EndBefore(cf) => task
                    .get_end()
                    .and_then(|d| Date::from_abs(d.as_str()))
                    .filter(|d| d.before(cf))
                    .is_some(),
                FilterPair::EntryBefore(cf) => task
                    .get_entry()
                    .and_then(|d| Date::from_abs(d.as_str()))
                    .filter(|d| d.before(cf))
                    .is_some(),
                FilterPair::ValueBefore(k, cf) => task
                    .get_value(k)
                    .and_then(|d| Date::from_abs(d.as_str()))
                    .filter(|d| d.before(cf))
                    .is_some(),
                FilterPair::EndAfter(cf) => task
                    .get_end()
                    .and_then(|d| Date::from_abs(d.as_str()))
                    .filter(|d| d.after(cf))
                    .is_some(),
                FilterPair::EntryAfter(cf) => task
                    .get_entry()
                    .and_then(|d| Date::from_abs(d.as_str()))
                    .filter(|d| d.after(cf))
                    .is_some(),
                FilterPair::ValueAfter(k, cf) => task
                    .get_value(k)
                    .and_then(|d| Date::from_abs(d.as_str()))
                    .filter(|d| d.after(cf))
                    .is_some(),
                FilterPair::EndIn(cf) => task
                    .get_end()
                    .and_then(|d| Date::from_abs(d.as_str()))
                    .filter(|d| d.within(cf))
                    .is_some(),
                FilterPair::EntryIn(cf) => task
                    .get_entry()
                    .and_then(|d| Date::from_abs(d.as_str()))
                    .filter(|d| d.within(cf))
                    .is_some(),
                FilterPair::ValueIn(k, cf) => task
                    .get_value(k)
                    .and_then(|d| Date::from_abs(d.as_str()))
                    .filter(|d| d.within(cf))
                    .is_some(),
            },
            Kind::MatchesRegex(regex) => regex.is_match(task.as_str()),
            Kind::WithinPriorityRange(range) => task
                .get_priority()
                .filter(|pri| range.contains(pri))
                .is_some(),
            Kind::WithinNumberRange(range) => range.contains(&nr),
            Kind::WithinNumberList(list) => list.contains(&nr),
        };

        match self.negate {
            true => !result,
            false => result,
        }
    }

    pub fn conflicts(&self, other: &Filter) -> bool {
        match (&self.kind, &other.kind) {
            (Kind::All, _) => true,
            (_, Kind::All) => true,
            (Kind::IsCompleted, Kind::IsCompleted) => true,
            (Kind::HasProject(a), Kind::HasProject(b)) => a == b,
            (Kind::HasContext(a), Kind::HasContext(b)) => a == b,
            (Kind::HasPair(a), Kind::HasPair(b)) => a.stringify_key() == b.stringify_key(),
            (Kind::MatchesRegex(_), Kind::MatchesRegex(_)) => true,
            (Kind::WithinPriorityRange(_), Kind::WithinPriorityRange(_)) => true,
            (Kind::WithinNumberRange(_), Kind::WithinNumberRange(_)) => true,
            (Kind::WithinNumberList(_), _) => true,
            (_, Kind::WithinNumberList(_)) => true,
            _ => false,
        }
    }
}

impl<'a> FilterPair<'a> {
    const KEY_END: &'static str = "end";
    const KEY_PRI: &'static str = "pri";
    const KEY_ENTRY: &'static str = "entry";

    const MOD_ANY: &'static str = "any";
    const MOD_NONE: &'static str = "none";
    const MOD_BEFORE: &'static str = "before";
    const MOD_AFTER: &'static str = "after";
    const MOD_IN: &'static str = "in";

    pub fn new(str: &'a str, now: &Date, date_keys: &[Key]) -> Result<Option<Self>> {
        let (key, xmod, value) = match str.split_once(':') {
            None => return Ok(None),
            Some((key_mod, value)) => match key_mod.split_once('.') {
                Some((key, xmod)) => (key, Some(xmod), value),
                None => (key_mod, None, value),
            },
        };

        let is_date_key = date_keys.iter().any(|k| k == &Key::new(key));

        Ok(Some(match (key, xmod) {
            (FilterPair::KEY_END, None) => FilterPair::End(value.to_owned()),
            (FilterPair::KEY_PRI, None) => {
                match Some(value)
                    .filter(|v| v.len() == 1)
                    .and_then(|v| v.chars().next())
                    .and_then(Priority::from_char)
                {
                    Some(pri) => FilterPair::Priority(pri),
                    None => return Err(InvalidPriority(str.to_owned())),
                }
            }
            (FilterPair::KEY_ENTRY, None) => FilterPair::Entry(value.to_owned()),
            (_, None) => FilterPair::KeyValue(Key::new(key), value.to_owned()),
            (FilterPair::KEY_END, Some(FilterPair::MOD_ANY)) => FilterPair::AnyEnd,
            (FilterPair::KEY_PRI, Some(FilterPair::MOD_ANY)) => FilterPair::AnyPriority,
            (FilterPair::KEY_ENTRY, Some(FilterPair::MOD_ANY)) => FilterPair::AnyEntry,
            (_, Some(FilterPair::MOD_ANY)) => FilterPair::AnyValue(Key::new(key)),
            (FilterPair::KEY_END, Some(FilterPair::MOD_NONE)) => FilterPair::NoEnd,
            (FilterPair::KEY_PRI, Some(FilterPair::MOD_NONE)) => FilterPair::NoPriority,
            (FilterPair::KEY_ENTRY, Some(FilterPair::MOD_NONE)) => FilterPair::NoEntry,
            (_, Some(FilterPair::MOD_NONE)) => FilterPair::NoKey(Key::new(key)),
            (FilterPair::KEY_END, Some(FilterPair::MOD_BEFORE)) => match Date::new(value, now) {
                Some(date) => FilterPair::EndBefore(date),
                None => return Err(ModExpectsDateValue(str.to_string())),
            },
            (FilterPair::KEY_PRI, Some(FilterPair::MOD_BEFORE)) => {
                return Err(ModExpectsDateKey(str.to_string()))
            }
            (FilterPair::KEY_ENTRY, Some(FilterPair::MOD_BEFORE)) => match Date::new(value, now) {
                Some(date) => FilterPair::EntryBefore(date),
                None => return Err(ModExpectsDateValue(str.to_string())),
            },
            (_, Some(FilterPair::MOD_BEFORE)) if !is_date_key => {
                return Err(ModExpectsDateKey(str.to_string()))
            }
            (_, Some(FilterPair::MOD_BEFORE)) => match Date::new(value, now) {
                Some(date) => FilterPair::ValueBefore(Key::new(key), date),
                None => return Err(ModExpectsDateValue(str.to_string())),
            },
            (FilterPair::KEY_END, Some(FilterPair::MOD_AFTER)) => match Date::new(value, now) {
                Some(date) => FilterPair::EndAfter(date),
                None => return Err(ModExpectsDateValue(str.to_string())),
            },
            (FilterPair::KEY_PRI, Some(FilterPair::MOD_AFTER)) => {
                return Err(ModExpectsDateKey(str.to_string()))
            }
            (FilterPair::KEY_ENTRY, Some(FilterPair::MOD_AFTER)) => match Date::new(value, now) {
                Some(date) => FilterPair::EntryAfter(date),
                None => return Err(ModExpectsDateValue(str.to_string())),
            },
            (_, Some(FilterPair::MOD_AFTER)) if !is_date_key => {
                return Err(ModExpectsDateKey(str.to_string()))
            }
            (_, Some(FilterPair::MOD_AFTER)) => match Date::new(value, now) {
                Some(date) => FilterPair::ValueAfter(Key::new(key), date),
                None => return Err(ModExpectsDateValue(str.to_string())),
            },
            (FilterPair::KEY_END, Some(FilterPair::MOD_IN)) => match Date::new(value, now) {
                Some(date) => FilterPair::EndIn(date),
                None => return Err(ModExpectsDateValue(str.to_string())),
            },
            (FilterPair::KEY_PRI, Some(FilterPair::MOD_IN)) => {
                return Err(ModExpectsDateKey(str.to_string()))
            }
            (FilterPair::KEY_ENTRY, Some(FilterPair::MOD_IN)) => match Date::new(value, now) {
                Some(date) => FilterPair::EntryIn(date),
                None => return Err(ModExpectsDateValue(str.to_string())),
            },
            (_, Some(FilterPair::MOD_IN)) if !is_date_key => {
                return Err(ModExpectsDateKey(str.to_string()))
            }
            (_, Some(FilterPair::MOD_IN)) => match Date::new(value, now) {
                Some(date) => FilterPair::ValueIn(Key::new(key), date),
                None => return Err(ModExpectsDateValue(str.to_string())),
            },
            (_, Some(_)) => return Err(InvalidMod(str.to_string())),
        }))
    }

    fn stringify_key(&self) -> &str {
        match self {
            FilterPair::End(_) => FilterPair::KEY_END,
            FilterPair::AnyEnd => FilterPair::KEY_END,
            FilterPair::NoEnd => FilterPair::KEY_END,
            FilterPair::EndBefore(_) => FilterPair::KEY_END,
            FilterPair::EndAfter(_) => FilterPair::KEY_END,
            FilterPair::EndIn(_) => FilterPair::KEY_END,

            FilterPair::Priority(_) => FilterPair::KEY_PRI,
            FilterPair::AnyPriority => FilterPair::KEY_PRI,
            FilterPair::NoPriority => FilterPair::KEY_PRI,

            FilterPair::Entry(_) => FilterPair::KEY_ENTRY,
            FilterPair::AnyEntry => FilterPair::KEY_ENTRY,
            FilterPair::NoEntry => FilterPair::KEY_ENTRY,
            FilterPair::EntryBefore(_) => FilterPair::KEY_ENTRY,
            FilterPair::EntryAfter(_) => FilterPair::KEY_ENTRY,
            FilterPair::EntryIn(_) => FilterPair::KEY_ENTRY,

            FilterPair::KeyValue(k, _) => k.as_str(),
            FilterPair::AnyValue(k) => k.as_str(),
            FilterPair::NoKey(k) => k.as_str(),
            FilterPair::ValueBefore(k, _) => k.as_str(),
            FilterPair::ValueAfter(k, _) => k.as_str(),
            FilterPair::ValueIn(k, _) => k.as_str(),
        }
    }
}
