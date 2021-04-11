use crate::color::*;
use crate::field::*;
use crate::task::Task;
use crate::token::Token;
use std::io::Write;

pub trait Print {
    fn print(&self, stdout: &mut String, print_color: bool);
}

impl Print for Bg {
    fn print(&self, stdout: &mut String, print_color: bool) {
        if !print_color || Bg::get() == *self {
            return;
        }
        Bg::set(self);
        match self {
            Bg::Default => stdout.push_str("\x1b[0m"),
            Bg::Custom(code) => {
                let mut buf = [0u8; Number::from_usize(usize::MAX).digits()];
                stdout.push_str("\x1b[48;5;");
                stdout.push_str(local_fmt_nr(&mut buf, *code as usize));
                stdout.push('m');
            }
        }
    }
}

impl Print for Fg {
    fn print(&self, stdout: &mut String, print_color: bool) {
        if !print_color || Fg::get() == *self {
            return;
        }
        Fg::set(self);
        match self {
            Fg::Black => stdout.push_str("\x1b[38;5;0m"),
            Fg::Blue => stdout.push_str("\x1b[38;5;33m"),
            Fg::Cyan => stdout.push_str("\x1b[38;5;6m"),
            Fg::Green => stdout.push_str("\x1b[38;5;2m"),
            Fg::LightGray => stdout.push_str("\x1b[38;5;246m"),
            Fg::Magenta => stdout.push_str("\x1b[38;5;092m"),
            Fg::Red => stdout.push_str("\x1b[38;5;196m"),
            Fg::White => stdout.push_str("\x1b[38;5;255m"),
            Fg::Yellow => stdout.push_str("\x1b[38;5;11m"),
            Fg::Default => stdout.push_str("\x1b[0m"),
        }
    }
}

impl Print for Annotation {
    fn print(&self, stdout: &mut String, print_color: bool) {
        Fg::Yellow.print(stdout, print_color);
        stdout.push('|');
    }
}

impl<'a> Print for Context<'a> {
    fn print(&self, stdout: &mut String, print_color: bool) {
        Fg::Blue.print(stdout, print_color);
        stdout.push_str(self.as_str());
    }
}

impl<'a> Print for End<'a> {
    fn print(&self, stdout: &mut String, print_color: bool) {
        Fg::LightGray.print(stdout, print_color);
        stdout.push_str(self.as_str());
    }
}

impl<'a> Print for Entry<'a> {
    fn print(&self, stdout: &mut String, print_color: bool) {
        Fg::LightGray.print(stdout, print_color);
        stdout.push_str(self.as_str());
    }
}

impl<'a> Print for Key<'a> {
    fn print(&self, stdout: &mut String, print_color: bool) {
        Fg::Green.print(stdout, print_color);
        stdout.push_str(self.as_str());
        Fg::LightGray.print(stdout, print_color);
        stdout.push(':');
    }
}

impl Print for Marker {
    fn print(&self, stdout: &mut String, print_color: bool) {
        Fg::LightGray.print(stdout, print_color);
        stdout.push('x');
    }
}

impl<'a> Print for Normal<'a> {
    fn print(&self, stdout: &mut String, print_color: bool) {
        Fg::White.print(stdout, print_color);
        stdout.push_str(self.as_str());
    }
}

impl Print for Number {
    fn print(&self, stdout: &mut String, print_color: bool) {
        Fg::LightGray.print(stdout, print_color);
        let mut buf = [0u8; Number::from_usize(usize::MAX).digits()];
        stdout.push_str(local_fmt_nr(&mut buf, self.as_usize()));
    }
}

impl<'a> Print for Pair<'a> {
    fn print(&self, stdout: &mut String, print_color: bool) {
        self.key.print(stdout, print_color);
        self.value.print(stdout, print_color);
    }
}

impl Print for Priority {
    fn print(&self, stdout: &mut String, print_color: bool) {
        // Map priority to answer color in rough order of how attention grabbing it is.
        const ORDER: &[u8] = &[1, 9, 3, 11, 5, 13, 6, 14, 2, 10, 4, 12, 0, 8];
        const BLOCKSIZE: usize = ('Z' as usize - 'A' as usize + ORDER.len()) / ORDER.len();
        let code = ORDER[(self.as_u8() as usize - 'A' as usize) / BLOCKSIZE];
        let bg = Bg::Custom(code);
        let fg = if code < 8 && code != 3 && code != 6 {
            Fg::White
        } else {
            Fg::Black
        };
        let previous_bg = Bg::get();
        bg.print(stdout, print_color);
        fg.print(stdout, print_color);
        stdout.push('(');
        stdout.push(self.as_u8() as char);
        stdout.push(')');
        previous_bg.print(stdout, print_color);
    }
}

impl<'a> Print for Project<'a> {
    fn print(&self, stdout: &mut String, print_color: bool) {
        Fg::Magenta.print(stdout, print_color);
        stdout.push_str(self.as_str());
    }
}

impl<'a> Print for Space<'a> {
    fn print(&self, stdout: &mut String, _print_color: bool) {
        stdout.push_str(self.as_str());
    }
}

impl<'a> Print for Value<'a> {
    fn print(&self, stdout: &mut String, print_color: bool) {
        Fg::Cyan.print(stdout, print_color);
        stdout.push_str(self.as_str());
    }
}

impl<'a> Print for Task<'a> {
    fn print(&self, stdout: &mut String, print_color: bool) {
        for (token, _) in self.iter() {
            token.print(stdout, print_color);
        }
    }
}

impl<'a> Print for Token<'a> {
    fn print(&self, stdout: &mut String, print_color: bool) {
        match self {
            Token::Annotation(v) => v.print(stdout, print_color),
            Token::Context(v) => v.print(stdout, print_color),
            Token::End(v) => v.print(stdout, print_color),
            Token::Entry(v) => v.print(stdout, print_color),
            Token::Key(v) => v.print(stdout, print_color),
            Token::Marker(v) => v.print(stdout, print_color),
            Token::Normal(v) => v.print(stdout, print_color),
            Token::Pair(v) => v.print(stdout, print_color),
            Token::Priority(v) => v.print(stdout, print_color),
            Token::Project(v) => v.print(stdout, print_color),
            Token::Space(v) => v.print(stdout, print_color),
        }
    }
}

fn local_fmt_nr(buf: &mut [u8; Number::from_usize(usize::MAX).digits()], usize: usize) -> &str {
    // If the `&mut` is dropped as clippy requests, the buffer is moved
    // and unavailable for the from_utf8_unchecked() access
    #[allow(clippy::redundant_slicing)]
    let mut cursor = std::io::Cursor::new(&mut buf[..]);
    let _ = write!(cursor, "{}", usize);
    let len = cursor.position() as usize;
    unsafe { std::str::from_utf8_unchecked(&buf[..len]) }
}
