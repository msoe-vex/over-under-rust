use alloc::boxed::Box;

pub enum Event<'a> {
    OneTime(&'a dyn Fn() -> bool),
    Toggle(&'a dyn Fn() -> bool),
    While(&'a dyn Fn() -> bool),
}

impl Event<'_> {
    pub fn call(&self) -> bool {
        match self {
            Event::OneTime(callback) => callback(),
            Event::Toggle(callback) => callback(),
            Event::While(callback) => callback(),
        }
    }
}
