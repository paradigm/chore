use std::cell::Cell;

thread_local! {
    static CURRENT_FG: Cell<Fg> = Cell::new(Fg::Default);
    static CURRENT_BG: Cell<Bg> = Cell::new(Bg::Default);
}

#[derive(Clone, Copy, PartialEq)]
pub enum Bg {
    Default,
    Custom(u8),
}

#[derive(Clone, Copy, PartialEq)]
pub enum Fg {
    Black,
    Blue,
    Cyan,
    Default,
    Green,
    LightGray,
    Magenta,
    Red,
    White,
    Yellow,
}

impl Bg {
    pub fn get() -> Self {
        CURRENT_BG.with(|current| current.get())
    }

    pub fn set(&self) {
        CURRENT_BG.with(|current| current.set(*self))
    }
}

impl Fg {
    pub fn get() -> Self {
        CURRENT_FG.with(|current| current.get())
    }

    pub fn set(&self) {
        CURRENT_FG.with(|current| current.set(*self))
    }
}
