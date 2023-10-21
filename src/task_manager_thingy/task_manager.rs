use alloc::vec::Vec;

use super::{event::Event, task_node::TaskNode};

pub struct TaskManager {
    events: Vec<(Event, TaskNode)>,
    schedule: Vec<TaskNode>,
}

impl TaskManager {
    pub fn addEvent(event: Event, taskNode: TaskNode) {}

    pub fn periodic() {}
}
