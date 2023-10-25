use alloc::{collections::VecDeque, task, vec::Vec};

use super::{event::Event, task_node::TaskNode};

pub struct TaskManager<'a> {
    events: Vec<(Event<'a>, TaskNode)>,
    schedule: VecDeque<TaskNode>,
}

impl<'a> TaskManager<'a> {
    pub fn addEvent(&mut self, event: Event<'a>, task_node: TaskNode) {
        self.events.push((event, task_node));
    }

    pub fn periodic(mut self) {
        let mut events = Vec::new();
        while self.events.len() > 0 {
            let event_pair = self.events.pop().unwrap();
            match event_pair {
                (Event::OneTime(event), task_node) => {
                    if event() {
                        self.schedule(task_node);
                    } else {
                        events.push((event_pair.0, task_node))
                    }
                }

                (Event::Toggle(active, event), task_node) => {
                    if event() && !active {
                        self.schedule(task_node.clone());
                        events.push((Event::Toggle(true, event), task_node))
                    } else if !event() && active {
                        events.push((Event::Toggle(false, event), task_node))
                    } else {
                        events.push((event_pair.0, task_node))
                    }
                }

                (Event::While(event), task_node) => {
                    if event() {
                        self.schedule(task_node.clone());
                    }
                    events.push((event_pair.0, task_node))
                }
            }
        }

        self.events = events;

        //loop through tasks and
        //if not started: call start
        //if started: call periodic
        //if ended: call end
    }

    fn schedule(&mut self, task_node: TaskNode) {
        self.schedule.push_back(task_node);
    }
}
