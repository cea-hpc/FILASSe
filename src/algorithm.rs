use crate::job::Task;
use std::{collections::VecDeque, sync::Mutex};

pub trait Algorithm {
    fn get_next_task(&self, queue: &Mutex<VecDeque<Task>>) -> Option<Task>;
    fn add_to_queue(&self, queue: &Mutex<VecDeque<Task>>, task: Task);
}

#[derive(Debug)]
pub struct Fifo {}

impl Algorithm for Fifo {
    fn add_to_queue(&self, queue: &Mutex<VecDeque<Task>>, task: Task) {
        queue.lock().unwrap().push_back(task);
    }
    fn get_next_task(&self, queue: &Mutex<VecDeque<Task>>) -> Option<Task> {
        let mut guard = queue.lock().unwrap();
        guard.pop_front()
    }
}
