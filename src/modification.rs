use crate::date::Date;
use crate::error::*;
use crate::field::*;
use crate::task::TaskBuf;
use crate::token::Token;

pub enum Modification<'a> {
    SetPending,
    SetCompleted,
    AddProject(Project<'a>),
    RemoveProject(Project<'a>),
    AddContext(Context<'a>),
    RemoveContext(Context<'a>),
    SetPair(ModPair<'a>),
    Append(String),
    SetBody(String),
}

pub enum ModPair<'a> {
    SetEnd(String),
    SetPriority(Priority),
    SetEntry(String),
    SetValue(Key<'a>, String),
    RemoveEnd,
    RemovePriority,
    RemoveEntry,
    RemoveKey(Key<'a>),
}

pub struct ModOutput {
    pub add: Option<TaskBuf>,
    pub remove_similar: bool,
}

impl<'a> Modification<'a> {
    const DONE: &'static str = "+done";

    pub fn new(str: &'a str, now: &Date, date_keys: &[Key]) -> Result<Self> {
        let full = str;
        let (str, negate) = match str.strip_prefix('-').or_else(|| str.strip_prefix('!')) {
            Some(str) => (str, true),
            None => (str, false),
        };

        Ok(if str == Modification::DONE {
            match negate {
                false => Modification::SetCompleted,
                true => Modification::SetPending,
            }
        } else if let Some(proj) = Project::new(str) {
            match negate {
                true => Modification::RemoveProject(proj),
                false => Modification::AddProject(proj),
            }
        } else if let Some(ctx) = Context::new(str) {
            match negate {
                true => Modification::RemoveContext(ctx),
                false => Modification::AddContext(ctx),
            }
        } else if let Some(pair) = ModPair::new(str, negate, now, date_keys)? {
            Modification::SetPair(pair)
        } else if let Some(text) = full.strip_prefix(">>") {
            Modification::Append(text.to_owned())
        } else {
            Modification::SetBody(full.to_owned())
        })
    }

    pub fn apply(&self, task: &mut TaskBuf, date_keys: &[Key]) -> ModOutput {
        let mut add = None;
        let mut remove_similar = false;
        match self {
            Modification::SetPending => {
                task.set_end(None);
                task.set_pending();
            }
            Modification::SetCompleted => {
                if !task.as_task().is_completed() {
                    if let Some(off) = task.as_task().get_value(&Key::new("recur")) {
                        let mut new = task.clone();
                        let task = task.as_task();
                        if let Some(entry) =
                            task.get_entry().and_then(|e| Date::from_abs(e.as_str()))
                        {
                            if let Some(new_date) = Date::from_rel(off.as_str(), &entry) {
                                new.set_entry(Some(Entry(&new_date.to_string())))
                            }
                        }
                        for key in date_keys {
                            let value = match task
                                .get_value(key)
                                .and_then(|v| Date::from_abs(v.as_str()))
                            {
                                Some(value) => value,
                                None => continue,
                            };
                            if let Some(new_date) = Date::from_rel(off.as_str(), &value) {
                                new.set_value(key, &Value::new(&new_date.to_string()))
                            }
                        }
                        add = Some(new);
                    }
                    task.set_completed();
                    if task
                        .as_task()
                        .has_token(&Token::Project(Project::new("+update").unwrap()))
                    {
                        remove_similar = true;
                    }
                }
            }
            Modification::AddProject(proj) => {
                if !task.as_task().has_token(&Token::Project(proj.clone())) {
                    task.append_text(proj.as_str())
                }
            }
            Modification::RemoveProject(proj) => task.remove_token(&Token::Project(proj.clone())),
            Modification::AddContext(ctx) => {
                if !task.as_task().has_token(&Token::Context(ctx.clone())) {
                    task.append_text(ctx.as_str())
                }
            }
            Modification::RemoveContext(ctx) => task.remove_token(&Token::Context(ctx.clone())),
            Modification::SetPair(pair) => match pair {
                ModPair::SetEnd(v) => task.set_end(Some(End(v))),
                ModPair::SetPriority(v) => task.set_priority(Some(*v)),
                ModPair::SetEntry(v) => task.set_entry(Some(Entry(v))),
                ModPair::SetValue(k, v) => task.set_value(k, &Value::new(v)),
                ModPair::RemoveEnd => task.set_end(None),
                ModPair::RemovePriority => task.set_priority(None),
                ModPair::RemoveEntry => task.set_entry(None),
                ModPair::RemoveKey(k) => task.remove_pair(k),
            },
            Modification::Append(v) => task.append_text(v),
            Modification::SetBody(v) => {
                task.clear_body();
                task.append_text(v);
            }
        }

        ModOutput {
            add,
            remove_similar,
        }
    }
}

impl<'a> ModPair<'a> {
    const END: &'static str = "end";
    const PRI: &'static str = "pri";
    const ENTRY: &'static str = "entry";

    pub fn new(str: &'a str, negate: bool, now: &Date, date_keys: &[Key]) -> Result<Option<Self>> {
        let (key, value) = match str.split_once(':') {
            None => return Ok(None),
            Some((key, value)) => (key, value),
        };

        let is_date_key = date_keys.iter().any(|k| k == &Key::new(key));
        let empty_value = value.is_empty();

        Ok(Some(match (key, negate, empty_value) {
            (ModPair::END, false, false) => {
                match Date::new(value, now).map(|date| date.to_string()) {
                    Some(date) => match End::new(&date, Stage::End) {
                        Some(_) => ModPair::SetEnd(date),
                        None => return Err(InvalidEnd(str.to_owned())),
                    },
                    None => return Err(KeyExpectsDateValue(str.to_string())),
                }
            }
            (ModPair::PRI, false, false) => {
                match Some(value)
                    .filter(|v| v.len() == 1)
                    .and_then(|v| v.chars().next())
                    .and_then(Priority::from_char)
                {
                    Some(pri) => ModPair::SetPriority(pri),
                    None => return Err(InvalidPriority(str.to_owned())),
                }
            }
            (ModPair::ENTRY, false, false) => {
                match Date::new(value, now).map(|date| date.to_string()) {
                    Some(date) => match Entry::new(&date, Stage::Entry) {
                        Some(_) => ModPair::SetEntry(date),
                        None => return Err(InvalidEntry(str.to_owned())),
                    },
                    None => return Err(KeyExpectsDateValue(str.to_string())),
                }
            }
            (_, false, false) if is_date_key => match Date::new(value, now) {
                Some(date) => ModPair::SetValue(Key::new(key), date.to_string()),
                None => return Err(KeyExpectsDateValue(str.to_string())),
            },
            (_, false, false) => ModPair::SetValue(Key::new(key), value.to_owned()),
            (ModPair::END, false, true) => ModPair::RemoveEnd,
            (ModPair::PRI, false, true) => ModPair::RemovePriority,
            (ModPair::ENTRY, false, true) => ModPair::RemoveEntry,
            (_, false, true) if is_date_key => ModPair::RemoveKey(Key::new(key)),
            (_, false, true) => ModPair::SetValue(Key::new(key), value.to_owned()),
            (_, true, false) => return Err(CannotModNegateKeyValue(str.to_owned())),
            (ModPair::END, true, true) => ModPair::RemoveEnd,
            (ModPair::PRI, true, true) => ModPair::RemovePriority,
            (ModPair::ENTRY, true, true) => ModPair::RemoveEntry,
            (_, true, true) => ModPair::RemoveKey(Key::new(key)),
        }))
    }
}
