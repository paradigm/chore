use super::File;
use crate::command::Command;
use crate::date::Date;
use crate::error::*;
use crate::field::*;
use crate::filter::Filter;
use crate::modification::Modification;

#[derive(Debug, PartialEq)]
enum ArgStage {
    Filter,
    Command,
    Modification,
}

pub enum ArgNext<'a> {
    Filter(Filter<'a>),
    Command(Command),
    Modification(Modification<'a>),
}

pub struct ArgIter<'a> {
    stage: ArgStage,
    stack: Vec<&'a str>,
    force_append: bool,
    now: &'a Date,
    date_keys: &'a [Key<'a>],
    filter_aliases: &'a [File],
    command_aliases: &'a [File],
    modification_aliases: &'a [File],
}

impl<'a> ArgIter<'a> {
    pub fn new(
        args: &'a [String],
        now: &'a Date,
        date_keys: &'a [Key<'a>],
        filter_aliases: &'a [File],
        command_aliases: &'a [File],
        modification_aliases: &'a [File],
    ) -> Self {
        let mut stack = Vec::new();
        for arg in args {
            stack.insert(0, arg.as_ref());
        }

        ArgIter {
            stage: ArgStage::Filter,
            stack,
            force_append: false,
            now,
            date_keys,
            filter_aliases,
            command_aliases,
            modification_aliases,
        }
    }
}

impl<'a> Iterator for ArgIter<'a> {
    type Item = Result<ArgNext<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut arg = self.stack.pop()?;

        if self.stage == ArgStage::Filter {
            if let Some(File { content, .. }) = find_name(arg, self.filter_aliases) {
                let mut aliases = content.as_ref();
                let i = self.stack.len();
                while let Some((head, tail)) = split_token(aliases) {
                    self.stack.insert(i, head);
                    aliases = tail;
                }
                arg = self.stack.pop()?;
            }
        }

        while self.stage == ArgStage::Filter && is_all_whitespace(arg) {
            arg = self.stack.pop()?;
        }

        if self.stage == ArgStage::Filter {
            match Filter::new(arg, &self.now, &self.date_keys) {
                Err(err) => return Some(Err(err)),
                Ok(Some(filter)) => return Some(Ok(ArgNext::Filter(filter))),
                Ok(None) => self.stage = ArgStage::Command,
            }
        }

        if self.stage == ArgStage::Command {
            if let Some(File { content, .. }) = find_name(arg, self.command_aliases) {
                let mut aliases = content.as_ref();
                let i = self.stack.len();
                while let Some((head, tail)) = split_token(aliases) {
                    self.stack.insert(i, head);
                    aliases = tail;
                }
                arg = self.stack.pop()?;
            }
        }

        if self.stage == ArgStage::Command {
            self.stage = ArgStage::Modification;
            return match Command::new(arg) {
                Some(command) => Some(Ok(ArgNext::Command(command))),
                None => Some(Err(NotAFilterOrCommand(arg.to_owned()))),
            };
        }

        if let Some(File { content, .. }) = find_name(arg, self.modification_aliases) {
            let mut aliases = content.as_ref();
            let i = self.stack.len();
            while let Some((head, tail)) = split_token(aliases) {
                self.stack.insert(i, head);
                aliases = tail;
            }
            arg = self.stack.pop()?;
        }

        while !self.force_append && is_all_whitespace(arg) {
            arg = self.stack.pop()?;
        }

        if arg.contains(|c: char| c.is_ascii_whitespace())
            && arg.contains(|c: char| !c.is_ascii_whitespace())
        {
            let mut content = arg;
            let i = self.stack.len();
            while let Some((head, tail)) = split_token(content) {
                self.stack.insert(i, head);
                content = tail;
            }
            arg = self.stack.pop()?;
        }

        Some(match Modification::new(arg, &self.now, &self.date_keys) {
            Ok(Modification::Append(str)) => {
                self.force_append = true;
                Ok(ArgNext::Modification(Modification::Append(str)))
            }
            Ok(Modification::SetBody(str)) if !self.force_append => {
                self.force_append = true;
                Ok(ArgNext::Modification(Modification::SetBody(str)))
            }
            Ok(Modification::SetBody(str)) if self.force_append => {
                Ok(ArgNext::Modification(Modification::Append(str)))
            }
            Ok(modification) => {
                Ok(ArgNext::Modification(modification))
            }
            Err(err) => Err(err)
        })
    }
}

fn find_name<'a>(n: &str, fs: &'a [File]) -> Option<&'a File> {
    fs.iter().find(|File { name, .. }| name == n)
}

fn is_all_whitespace(str: &str) -> bool {
    str.chars().all(|c: char| c.is_ascii_whitespace())
}

fn split_token(str: &str) -> Option<(&str, &str)> {
    let str = str.trim_end();
    let head = match str.starts_with(|c: char| c.is_ascii_whitespace()) {
        true => str.split(|c: char| !c.is_ascii_whitespace()).next()?,
        false => str.split_ascii_whitespace().next()?,
    };

    let tail = str.get(head.len()..)?;

    Some((head, tail))
}
