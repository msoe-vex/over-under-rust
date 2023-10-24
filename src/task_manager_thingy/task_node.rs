use alloc::{boxed::Box, vec::Vec};

use super::task::Task;

pub type Subsystem = ();

pub enum TaskNode {
    ParallelTaskGroup(Vec<TaskNode>),
    SequentialTaskGroup(Vec<TaskNode>),
    //TODO: make task contain the type for subsystems
    Task(Box<dyn Task>),
}

impl TaskNode {
    pub fn new(subsystem: Subsystem) {
        todo!()
    }

    pub fn is_done() -> bool {
        todo!()
    }

    pub fn start() {
        todo!()
    }

    pub fn periodic() {
        todo!()
    }

    pub fn end() {
        todo!()
    }
}
