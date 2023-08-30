use crate::{algorithm::Fifo, scheduler::Scheduler};
use std::{collections::VecDeque, sync::Mutex};

#[derive(Debug)]
pub struct SchedulerUser {
    pub scheduler: Scheduler,
}

impl SchedulerUser {
    pub fn create_task<F: Fn()>(&self, func: F) {
        self.scheduler.create_task(func);
    }

    pub fn join(&self) {
        self.yield_task();
    }

    pub fn yield_task(&self) {
        self.scheduler.yield_task();
    }

    pub fn lock(&self) {
        self.scheduler.lock();
    }
    pub fn unlock(&self) {
        self.scheduler.unlock();
    }
}

pub static SCHEDULER: SchedulerUser = SchedulerUser {
    scheduler: Scheduler {
        queue: Mutex::new(VecDeque::new()),
        algorithm: Fifo {},
        current: Mutex::new(None),
        idle: Mutex::new(None),
    },
};
unsafe impl Sync for SchedulerUser {}
