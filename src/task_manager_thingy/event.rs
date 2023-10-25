use alloc::boxed::Box;

pub enum Event<'a> {
    OneTime(&'a dyn Fn() -> bool),
    Toggle(bool, &'a dyn Fn() -> bool),
    While(&'a dyn Fn() -> bool),
}
