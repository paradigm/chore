//
// This file is just an I/O driver for the purely functional rest of the code base.
//

type Result<T> = std::result::Result<T, err::Error>;
use chore::Config;
use chore::Output;
use err::Error::*;

macro_rules! path {
    ($h:ident, $f: literal) => {
        [&$h, concat!(".chore/", $f)].iter().collect()
    };
}

fn main() -> Result<()> {
    let home = std::env::var("HOME").map_err(|e| EnvVarErr("HOME".to_string(), e))?;

    let config = Config {
        args: std::env::args().skip(1).collect::<Vec<String>>(),
        now: chrono::Local::now().naive_local(),
        tasks: io::read_file(path!(home, "tasks"))?,
        undo: io::read_file(path!(home, "undo"))?,
        date_keys: io::read_file(path!(home, "date-keys"))?,
        filter_aliases: io::read_dir(path!(home, "filter-aliases"))?,
        command_aliases: io::read_dir(path!(home, "command-aliases"))?,
        modification_aliases: io::read_dir(path!(home, "modification-aliases"))?,
        default_filters: io::read_dir(path!(home, "default-filters"))?,
        print_color: unsafe { libc::isatty(libc::STDOUT_FILENO) == 1 },
    };

    match chore::run(config).map_err(ChoreErr)? {
        Output::JustPrint { stdout } => io::print(&stdout)?,
        Output::WriteFiles {
            stdout,
            confirm,
            tasks,
            undo,
        } => {
            io::print(&stdout)?;
            if confirm && !io::prompt()? {
                return Err(PromptDenied);
            }
            io::write_file(path!(home, "undo"), &undo)?;
            io::write_file(path!(home, "tasks"), &tasks)?;
        }
    }

    Ok(())
}

mod err {
    use std::path::PathBuf;
    use Error::*;
    pub enum Error {
        ChoreErr(chore::Error),
        DirReadErr(PathBuf, std::io::Error),
        EnvVarErr(String, std::env::VarError),
        FileCreateErr(PathBuf, std::io::Error),
        FileReadErr(PathBuf, std::io::Error),
        FileRenameErr(PathBuf, PathBuf, std::io::Error),
        FileWriteErr(PathBuf, std::io::Error),
        OsStrToStrErr(std::ffi::OsString),
        PromptDenied,
    }

    impl std::fmt::Debug for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ChoreErr(e) => write!(f, "{:?}", e),
                DirReadErr(v, e) => write!(f, "Unable to read directory `{:?}`: {}", v, e),
                EnvVarErr(v, e) => write!(f, "Unable to read environment variable `{}`: {}", v, e),
                FileCreateErr(v, e) => write!(f, "Unable to create file `{:?}`: {}", v, e),
                FileReadErr(v, e) => write!(f, "Unable to read file `{:?}`: {}", v, e),
                FileRenameErr(s, d, e) => {
                    writeln!(f, "Unable to rename {:?} over {:?}: {}", s, d, e)
                }
                FileWriteErr(v, e) => write!(f, "Unable to write file `{:?}`: {}", v, e),
                OsStrToStrErr(v) => write!(f, "Unable to convert `{:?}` to UTF-8 string", v),
                PromptDenied => writeln!(f, "Confirmation denied, aborting without changes"),
            }
        }
    }
}

mod io {
    type Result<T> = std::result::Result<T, super::err::Error>;
    use super::err::Error::*;
    use chore::File;
    use std::io::prelude::*;
    use std::path::PathBuf;

    pub fn read_file(path: PathBuf) -> Result<Option<String>> {
        match std::fs::read_to_string(&path) {
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(FileReadErr(path, err)),
            Ok(ok) => Ok(Some(ok)),
        }
    }

    pub fn read_dir(path: PathBuf) -> Result<Vec<File>> {
        let mut files = Vec::new();
        match std::fs::read_dir(&path) {
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => Ok(files),
                _ => Err(DirReadErr(path, err)),
            },
            Ok(dir) => {
                for entry in dir {
                    let entry = entry.map_err(|e| FileReadErr(path.clone(), e))?;
                    let path = entry.path();
                    files.push(File {
                        name: entry.file_name().into_string().map_err(OsStrToStrErr)?,
                        content: std::fs::read_to_string(&path)
                            .map_err(|e| FileReadErr(path, e))?,
                    });
                }
                Ok(files)
            }
        }
    }

    pub fn write_file(path: PathBuf, content: &str) -> Result<()> {
        let tmp_path = path.with_extension(format!("chore-tmp-{}", std::process::id()));
        let create_err = |e| FileCreateErr(tmp_path.clone(), e);
        let write_err = |e| FileWriteErr(tmp_path.clone(), e);
        {
            let mut file = std::fs::File::create(&tmp_path).map_err(create_err)?;
            std::io::BufWriter::with_capacity(content.len(), &file)
                .write_all(content.as_bytes())
                .map_err(write_err)?;
            file.flush().map_err(write_err)?;
        }
        std::fs::rename(&tmp_path, &path).map_err(|e| FileRenameErr(tmp_path, path, e))?;
        Ok(())
    }

    pub fn print(str: &str) -> Result<()> {
        let err = |e| FileWriteErr(PathBuf::from("/dev/stdout"), e);
        let stdout = std::io::stdout();
        let mut stdout = std::io::BufWriter::with_capacity(str.len(), stdout);
        match stdout.write_all(str.as_bytes()) {
            Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => {}
            Err(e) => return Err(e).map_err(err),
            Ok(_) => {}
        }
        stdout.flush().map_err(err)?;
        Ok(())
    }

    pub fn prompt() -> Result<bool> {
        print("apply changes? [y/N] ")?;
        Ok(matches!(
            std::io::stdin().lock().bytes().next(),
            Some(Ok(b'y'))
        ))
    }
}
