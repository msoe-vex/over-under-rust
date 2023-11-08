use alloc::{collections::VecDeque, task, vec::Vec};

use super::{event::Event, task_node::TaskNode};

pub struct TaskManager {
    events: Vec<(Event, TaskNode)>,
    schedule: VecDeque<TaskNode>,
}

impl TaskManager {
    pub fn addEvent(&mut self, event: Event, task_node: TaskNode) {
        self.events.push((event, task_node));
    }

    pub fn periodic(mut self) {
        self.events_periodic();
        self.scheduler_periodic();
        //loop through tasks and
        //if not started: call start
        //if started: call periodic
        //if ended: call end
    }

    fn events_periodic(&mut self) {
        let mut events = Vec::new();
        while self.events.len() > 0 {
            let event_pair = self.events.pop().unwrap();
            match event_pair {
                (Event::OneTime(event), task_node) => {
                    if event() {
                        self.schedule(task_node);
                    } else {
                        events.push((Event::OneTime(event), task_node))
                    }
                }

                (Event::Toggle(active, checkEventState), task_node) => {
                    if checkEventState() && !active {
                        self.schedule(task_node.clone());
                        events.push((Event::Toggle(true, checkEventState), task_node))
                    } else if !checkEventState() && active {
                        events.push((Event::Toggle(false, checkEventState), task_node))
                    } else {
                        events.push((Event::Toggle(active, checkEventState), task_node))
                    }
                }

                (Event::While(event), task_node) => {
                    if event() {
                        self.schedule(task_node.clone());
                    }
                    events.push((Event::While(event), task_node))
                }
            }
        }

        self.events = events;
    }

    fn scheduler_periodic(&mut self) {
        for i in (0..self.events.len()).rev() {}
    }

    fn schedule(&mut self, task_node: TaskNode) {
        self.schedule.push_back(task_node);
    }
}
