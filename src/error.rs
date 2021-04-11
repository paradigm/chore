use std::path::PathBuf;

pub enum Error {
    CannotModNegateKeyValue(String),
    CmdDisallowsMod,
    ConfPromptDeniedErr,
    DirReadErr(PathBuf, std::io::Error),
    EmptyUndo,
    EnvVarErr(String, std::env::VarError),
    FileCreateErr(PathBuf, std::io::Error),
    FileReadErr(PathBuf, std::io::Error),
    FileRenameErr(PathBuf, PathBuf, std::io::Error),
    FileWriteErr(PathBuf, std::io::Error),
    InvalidDefaultFilter(String),
    InvalidEnd(String),
    InvalidEntry(String),
    InvalidMod(String),
    InvalidPriority(String),
    InvalidRegex(String),
    KeyExpectsDateValue(String),
    MalformedUndo(String),
    ModExpectsDateKey(String),
    ModExpectsDateValue(String),
    NotAFilterOrCommand(String),
    OsStrToStrErr(std::ffi::OsString),
    UndoMismatch(String),
}
pub use Error::*;
pub type Result<T> = std::result::Result<T, Error>;

macro_rules! arg {
    ($f:ident, $a: ident, $msg:literal) => {
        writeln!($f, concat!("{:?} ", $msg), $a)
    };
}

macro_rules! args {
    ($f:ident, $a: ident, $b: ident, $msg:literal) => {
        writeln!($f, concat!($msg, " {:?}: {}"), $a, $b)
    };
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CannotModNegateKeyValue(a) => arg!(f, a, "cannot modify negated key:value; try -key:"),
            CmdDisallowsMod => writeln!(f, "specified command cannot take modifications"),
            ConfPromptDeniedErr => writeln!(f, "confirmation denied, aborting without changes"),
            DirReadErr(v, e) => args!(f, v, e, "unable to read directory"),
            EmptyUndo => writeln!(f, "undo file is empty or non-existent; cannot undo further"),
            EnvVarErr(v, e) => args!(f, v, e, "unable to read environment variable"),
            FileCreateErr(v, e) => args!(f, v, e, "unable to create file"),
            FileReadErr(v, e) => args!(f, v, e, "unable to read file"),
            FileRenameErr(s, d, e) => writeln!(f, "cannot rename {:?} over {:?}: {}", s, d, e),
            FileWriteErr(v, e) => args!(f, v, e, "unable to write file"),
            InvalidDefaultFilter(a) => arg!(f, a, "is an invalid default filter"),
            InvalidEnd(a) => arg!(f, a, "is an invalid end date; expects one-day resolution"),
            InvalidEntry(a) => arg!(f, a, "is an invalid entry date; expects one-day resolution"),
            InvalidMod(a) => arg!(f, a, "has an invalid .mod:"),
            InvalidPriority(a) => arg!(f, a, "is not a valid priority A-Z"),
            InvalidRegex(a) => arg!(f, a, "starts with a '/' but is not valid regex"),
            KeyExpectsDateValue(a) => arg!(f, a, "contains date key but non-date value"),
            MalformedUndo(v) => writeln!(f, "undo file contains non-undo line: `{}`", v),
            ModExpectsDateKey(a) => arg!(f, a, "contains non-date key, conflicting with .mod:"),
            ModExpectsDateValue(a) => arg!(f, a, "contains non-date value, conflicting with .mod:"),
            NotAFilterOrCommand(a) => arg!(f, a, "is not a valid filter or command"),
            OsStrToStrErr(v) => arg!(f, v, "unable to convert to UTF-8 string"),
            UndoMismatch(v) => writeln!(f, "unable to find `{}` in task file to undo", v),
        }
    }
}
