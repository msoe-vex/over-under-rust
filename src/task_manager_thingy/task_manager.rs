use alloc::vec::Vec;

use super::{event::Event, task_node::TaskNode};

pub struct TaskManager<'a> {
    events: Vec<(Event<'a>, TaskNode)>,
    schedule: Vec<TaskNode>,
}

impl TaskManager<'_> {
    pub fn addEvent(event: Event, taskNode: TaskNode) {}

    pub fn periodic() {}
}
