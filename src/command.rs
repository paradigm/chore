use crate::color::*;
use crate::error::*;
use crate::field::*;
use crate::filter::Filter;
use crate::modification::{ModOutput, Modification};
use crate::print::*;
use crate::task::{Task, TaskBuf};
use crate::token::Token;
use crate::Output;
use std::collections::HashSet;

pub enum Command {
    ListTasks,
    ListProjects,
    ListContexts,
    ListKeys,
    AddTask,
    RemoveTasks,
    ModifyTasks,
    Undo,
}

enum TokenType {
    Projects,
    Contexts,
    Keys,
}

impl<'a> Command {
    const LIST: &'static str = "list";
    const PROJECTS: &'static str = "projects";
    const CONTEXTS: &'static str = "contexts";
    const KEYS: &'static str = "keys";
    const ADD: &'static str = "add";
    const DELETE: &'static str = "delete";
    const MODIFY: &'static str = "modify";
    const UNDO: &'static str = "undo";

    pub fn new(str: &'a str) -> Option<Self> {
        match str {
            Command::LIST => Some(Command::ListTasks),
            Command::PROJECTS => Some(Command::ListProjects),
            Command::CONTEXTS => Some(Command::ListContexts),
            Command::KEYS => Some(Command::ListKeys),
            Command::ADD => Some(Command::AddTask),
            Command::DELETE => Some(Command::RemoveTasks),
            Command::MODIFY => Some(Command::ModifyTasks),
            Command::UNDO => Some(Command::Undo),
            _ => None,
        }
    }

    pub fn run(
        &self,
        tasks: String,
        undo: String,
        filters: &[Filter],
        mods: &[Modification],
        date_keys: &'a [Key<'a>],
        print_color: bool,
    ) -> Result<Output> {
        match self {
            Command::ListTasks => {
                if !mods.is_empty() {
                    return Err(CmdDisallowsMod);
                }
                Command::list_tasks(tasks, filters, print_color)
            }
            Command::ListProjects => {
                if !mods.is_empty() {
                    return Err(CmdDisallowsMod);
                }
                Command::list_tokens(tasks, filters, TokenType::Projects, print_color)
            }
            Command::ListContexts => {
                if !mods.is_empty() {
                    return Err(CmdDisallowsMod);
                }
                Command::list_tokens(tasks, filters, TokenType::Contexts, print_color)
            }
            Command::ListKeys => {
                if !mods.is_empty() {
                    return Err(CmdDisallowsMod);
                }
                Command::list_tokens(tasks, filters, TokenType::Keys, print_color)
            }
            Command::AddTask => Command::add_tasks(tasks, undo, mods, print_color),
            Command::RemoveTasks => {
                if !mods.is_empty() {
                    return Err(CmdDisallowsMod);
                }
                Command::remove_tasks(tasks, undo, filters, print_color)
            }
            Command::ModifyTasks => {
                Command::modify_tasks(tasks, undo, filters, mods, date_keys, print_color)
            }
            Command::Undo => {
                if !mods.is_empty() {
                    return Err(CmdDisallowsMod);
                }
                Command::undo(tasks, undo, print_color)
            }
        }
    }

    fn list_tasks(tasks: String, filters: &[Filter], print_color: bool) -> Result<Output> {
        let tasks = tasks
            .lines()
            .enumerate()
            .map(|(nr, line)| (Number::from_enumerate(nr), Task::new(line)))
            .filter(|(nr, t)| filters.iter().all(|f| f.keep(t, *nr)))
            .collect::<Vec<_>>();

        let mut stdout = String::new();
        let max_nr_digits = match tasks.last() {
            Some((nr, _)) => nr.digits(),
            None => return Ok(Output::JustPrint { stdout }),
        };

        for (nr, task) in tasks {
            for _ in 0..(max_nr_digits - nr.digits()) {
                stdout.push(' ');
            }
            nr.print(&mut stdout, print_color);
            stdout.push(' ');

            task.print(&mut stdout, print_color);
            stdout.push('\n');
        }

        Bg::Default.print(&mut stdout, print_color);
        Ok(Output::JustPrint { stdout })
    }

    fn list_tokens(
        tasks: String,
        filters: &[Filter],
        tt: TokenType,
        print_color: bool,
    ) -> Result<Output> {
        let mut tokens = tasks
            .lines()
            .enumerate()
            .map(|(nr, line)| (Number::from_enumerate(nr), Task::new(line)))
            .filter(|(nr, t)| filters.iter().all(|f| f.keep(t, *nr)))
            .flat_map(|(_, task)| task.into_iter())
            .filter_map(|(token, _)| match (&tt, token) {
                (TokenType::Projects, Token::Project(t)) => Some(Token::Project(t)),
                (TokenType::Contexts, Token::Context(t)) => Some(Token::Context(t)),
                (TokenType::Keys, Token::Pair(Pair { key, .. })) => Some(Token::Key(key)),
                _ => None,
            })
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        let mut stdout = String::new();
        tokens.sort_unstable();

        for token in tokens {
            token.print(&mut stdout, print_color);
            stdout.push('\n');
        }
        Fg::Default.print(&mut stdout, print_color);

        Bg::Default.print(&mut stdout, print_color);
        Ok(Output::JustPrint { stdout })
    }

    fn add_tasks(
        tasks: String,
        mut undo: String,
        mods: &[Modification],
        print_color: bool,
    ) -> Result<Output> {
        let mut retained = tasks
            .lines()
            .map(|line| line.to_owned())
            .collect::<Vec<_>>();

        let mut buf = TaskBuf::new(String::new());
        let mut stdout = String::new();
        undo.push_str("---\n");

        for m in mods {
            let _ = match m {
                Modification::SetBody(body) => {
                    Modification::Append(body.clone()).apply(&mut buf, &[])
                }
                _ => m.apply(&mut buf, &[]),
            };
        }
        retained.push(buf.as_str().to_owned());

        Fg::Green.print(&mut stdout, print_color);
        stdout.push_str("ADD ");
        buf.as_task().print(&mut stdout, print_color);
        stdout.push('\n');

        undo.push_str("ADD ");
        undo.push_str(buf.as_str());
        undo.push('\n');

        retained.sort_unstable();
        let mut tasks = retained.join("\n");
        if !retained.is_empty() {
            tasks.push('\n');
        }

        Bg::Default.print(&mut stdout, print_color);
        Ok(Output::WriteFiles {
            stdout,
            confirm: false,
            tasks,
            undo,
        })
    }

    fn remove_tasks(
        tasks: String,
        mut undo: String,
        filters: &[Filter],
        print_color: bool,
    ) -> Result<Output> {
        let removed = tasks
            .lines()
            .enumerate()
            .map(|(nr, line)| (Number::from_enumerate(nr), Task::new(line)))
            .filter(|(nr, t)| filters.iter().all(|f| f.keep(t, *nr)))
            .map(|(_, task)| task)
            .collect::<Vec<_>>();
        let mut retained = tasks
            .lines()
            .enumerate()
            .map(|(nr, line)| (Number::from_enumerate(nr), Task::new(line)))
            .filter(|(nr, t)| !filters.iter().all(|f| f.keep(t, *nr)))
            .map(|(_, task)| task.into_str())
            .collect::<Vec<_>>();

        let mut stdout = String::new();
        undo.push_str("---\n");

        for task in &removed {
            Fg::Red.print(&mut stdout, print_color);
            stdout.push_str("DEL ");
            task.print(&mut stdout, print_color);
            stdout.push('\n');

            undo.push_str("DEL ");
            undo.push_str(task.as_str());
            undo.push('\n');
        }
        Fg::Default.print(&mut stdout, print_color);

        retained.sort_unstable();
        let mut tasks = retained.join("\n");
        if !retained.is_empty() {
            tasks.push('\n');
        }

        Bg::Default.print(&mut stdout, print_color);
        Ok(Output::WriteFiles {
            stdout,
            confirm: removed.len() > 1,
            tasks,
            undo,
        })
    }

    fn modify_tasks(
        tasks: String,
        mut undo: String,
        filters: &[Filter],
        mods: &[Modification],
        date_keys: &'a [Key<'a>],
        print_color: bool,
    ) -> Result<Output> {
        let modified = tasks
            .lines()
            .enumerate()
            .map(|(nr, line)| (Number::from_enumerate(nr), Task::new(line)))
            .filter(|(nr, t)| filters.iter().all(|f| f.keep(t, *nr)))
            .map(|(_, task)| task)
            .collect::<Vec<_>>();
        let mut retained = tasks
            .lines()
            .enumerate()
            .map(|(nr, line)| (Number::from_enumerate(nr), Task::new(line)))
            .filter(|(nr, t)| !filters.iter().all(|f| f.keep(t, *nr)))
            .map(|(_, task)| task.into_str().to_owned())
            .collect::<Vec<_>>();
        let mut added: Vec<String> = Vec::new();

        let mut buf = TaskBuf::new(String::new());
        let mut changed = 0;
        let mut stdout = String::new();
        undo.push_str("---\n");

        // TODO: refactor, especially remove_similar
        for task in modified {
            buf.set_to(&task);
            let mut add: Option<TaskBuf> = None;
            let mut remove_similar = false;

            // TODO: could probably do this with an iterator and early-exit
            for m in mods {
                let ModOutput {
                    add: a,
                    remove_similar: r,
                } = m.apply(&mut buf, date_keys);
                add = add.or(a);
                remove_similar = remove_similar || r;
            }
            if task == buf.as_task() {
                added.push(task.as_str().to_owned());
                continue;
            }

            added.push(buf.clone().into_string());
            changed += 1;

            Fg::Red.print(&mut stdout, print_color);
            stdout.push_str("DEL ");
            task.print(&mut stdout, print_color);
            stdout.push('\n');

            undo.push_str("DEL ");
            undo.push_str(task.as_str());
            undo.push('\n');

            if let Some(buf) = add {
                added.push(buf.as_str().to_owned());

                Fg::Green.print(&mut stdout, print_color);
                stdout.push_str("ADD ");
                buf.as_task().print(&mut stdout, print_color);
                stdout.push('\n');

                undo.push_str("ADD ");
                undo.push_str(buf.as_str());
                undo.push('\n');
            }

            if remove_similar {
                let mut remove = None;
                for (i, task) in retained.iter().enumerate() {
                    let mut buf_len = 0;
                    let mut task_len = 0;
                    let mut mismatch = false;
                    for ((a, ar), (b, br)) in buf.as_task().iter().zip(Task::new(&task).iter()) {
                        buf_len = ar.end;
                        task_len = br.end;
                        if a == b {
                            continue;
                        }
                        match (a, b) {
                            (Token::End(_), Token::End(_)) => continue,
                            (Token::Entry(_), Token::Entry(_)) => continue,
                            (
                                Token::Pair(Pair { key: ak, .. }),
                                Token::Pair(Pair { key: bk, .. }),
                            ) if date_keys.contains(&ak) && ak == bk => continue,
                            _ => {
                                mismatch = true;
                                break;
                            }
                        }
                    }
                    if !mismatch && buf.as_str().len() == buf_len && task.as_str().len() == task_len
                    {
                        remove = Some(i);
                        break;
                    }
                }
                if let Some(i) = remove {
                    Fg::Red.print(&mut stdout, print_color);
                    stdout.push_str("DEL ");
                    Task::new(&retained[i]).print(&mut stdout, print_color);
                    stdout.push('\n');

                    undo.push_str("DEL ");
                    undo.push_str(&retained[i]);
                    undo.push('\n');

                    retained.swap_remove(i);
                }
            }

            Fg::Green.print(&mut stdout, print_color);
            stdout.push_str("ADD ");
            buf.as_task().print(&mut stdout, print_color);
            stdout.push('\n');

            undo.push_str("ADD ");
            undo.push_str(buf.as_str());
            undo.push('\n');
        }

        retained.append(&mut added);
        retained.sort_unstable();
        let mut tasks = retained.join("\n");
        if !retained.is_empty() {
            tasks.push('\n');
        }

        Bg::Default.print(&mut stdout, print_color);
        Ok(Output::WriteFiles {
            stdout,
            confirm: changed > 1,
            tasks,
            undo,
        })
    }

    fn undo(tasks: String, undo: String, print_color: bool) -> Result<Output> {
        let mut tasks = tasks.lines().collect::<Vec<_>>();
        let mut stdout = String::new();
        let mut new_undo_len = undo.lines().count();

        for line in undo.lines().rev() {
            let head = line.split_ascii_whitespace().next();
            let tail = line.get(4..);
            let pos = match tail {
                Some(task) => tasks.iter().position(|t| t == &task),
                None => None,
            };

            new_undo_len -= 1;

            match (head, tail, pos) {
                (Some("---"), _, _) => break,
                (Some("ADD"), Some(line), Some(pos)) => {
                    Fg::Red.print(&mut stdout, print_color);
                    stdout.push_str("DEL ");
                    Task::new(line).print(&mut stdout, print_color);
                    stdout.push('\n');

                    tasks.remove(pos);
                }
                (Some("ADD"), Some(line), None) => {
                    return Err(UndoMismatch(line.to_string()));
                }
                (Some("DEL"), Some(line), _) => {
                    Fg::Green.print(&mut stdout, print_color);
                    stdout.push_str("ADD ");
                    Task::new(line).print(&mut stdout, print_color);
                    stdout.push('\n');

                    tasks.push(line);
                }
                _ => return Err(MalformedUndo(line.to_string())),
            }
        }

        tasks.sort_unstable();
        let need_trailing_newline = !tasks.is_empty();
        let mut tasks = tasks.join("\n");
        if need_trailing_newline {
            tasks.push('\n');
        }

        let undo = undo.lines().take(new_undo_len).collect::<Vec<_>>();
        let need_trailing_newline = !undo.is_empty();
        let mut undo = undo.join("\n");
        if need_trailing_newline {
            undo.push('\n');
        }

        Bg::Default.print(&mut stdout, print_color);
        Ok(Output::WriteFiles {
            stdout,
            confirm: true,
            tasks,
            undo,
        })
    }
}
