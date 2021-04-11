//
// Public interface for the core logic.  Entry point for both main.rs and integration tests.
//

mod args;
mod color;
mod command;
mod date;
mod error;
mod field;
mod filter;
mod modification;
mod print;
mod regex;
mod task;
mod taskiter;
mod token;
use crate::args::{ArgIter, ArgNext};
pub use error::*;

#[derive(Clone)]
pub struct Config {
    pub args: Vec<String>,
    pub now: chrono::NaiveDateTime,
    pub tasks: Option<String>,
    pub undo: Option<String>,
    pub date_keys: Option<String>,
    pub filter_aliases: Vec<File>,
    pub command_aliases: Vec<File>,
    pub modification_aliases: Vec<File>,
    pub default_filters: Vec<File>,
    pub print_color: bool,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            args: Vec::new(),
            now: chrono::Local::now().naive_local(),
            tasks: None,
            undo: None,
            date_keys: None,
            filter_aliases: Vec::new(),
            command_aliases: Vec::new(),
            modification_aliases: Vec::new(),
            default_filters: Vec::new(),
            print_color: false,
        }
    }
}

#[derive(Clone)]
pub struct File {
    pub name: String,
    pub content: String,
}

#[derive(Debug, PartialEq)]
pub enum Output {
    JustPrint {
        stdout: String,
    },
    WriteFiles {
        stdout: String,
        confirm: bool,
        tasks: String,
        undo: String,
    },
}

pub fn run(config: Config) -> Result<Output> {
    let now = date::Date::from_chrono(&config.now);
    let date_keys = match &config.date_keys.as_ref() {
        Some(date_keys) => date_keys
            .lines()
            .map(|str| match str.strip_suffix(':') {
                Some(str) => str,
                None => str,
            })
            .map(|str| field::Key::new(str))
            .collect::<Vec<_>>(),
        None => Vec::new(),
    };

    let args = ArgIter::new(
        &config.args,
        &now,
        &date_keys,
        &config.filter_aliases,
        &config.command_aliases,
        &config.modification_aliases,
    );

    let mut filters = Vec::new();
    let mut command = command::Command::ListTasks;
    let mut mods = Vec::new();

    for arg in args {
        match arg? {
            ArgNext::Filter(filter) => filters.push(filter),
            ArgNext::Command(new_command) => command = new_command,
            ArgNext::Modification(modification) => mods.push(modification),
        }
    }

    let mut default_filters = Vec::new();
    for File { content, .. } in &config.default_filters {
        for str in content.split_ascii_whitespace() {
            let filter = match filter::Filter::new(&str, &now, &date_keys)? {
                Some(filter) => filter,
                None => return Err(InvalidDefaultFilter(str.to_owned())),
            };
            if !filters.iter().any(|f| f.conflicts(&filter)) {
                default_filters.push(filter);
            }
        }
    }
    filters.append(&mut default_filters);

    let tasks = config.tasks.unwrap_or_else(|| "".to_owned());
    let undo = config.undo.unwrap_or_else(|| "".to_owned());

    command.run(tasks, undo, &filters, &mods, &date_keys, config.print_color)
}
