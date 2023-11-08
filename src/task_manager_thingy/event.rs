use alloc::{boxed::Box, rc::Rc};

#[derive(Clone)]
pub enum Event {
    OneTime(Rc<dyn Fn() -> bool>),
    Toggle(bool, Rc<dyn Fn() -> bool>),
    While(Rc<dyn Fn() -> bool>),
}
