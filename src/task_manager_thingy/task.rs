use super::task_node::Subsystem;

pub trait Task {
    fn is_done(&self) -> bool;

    fn start(&self);

    fn periodic(&self);

    fn end(&self);
}
    