use crate::{
    algorithm::{Algorithm, Fifo},
    job::{Job, Task},
};
use std::{collections::VecDeque, sync::Mutex};

#[derive(Debug)]
pub struct Scheduler {
    pub queue: Mutex<VecDeque<Task>>,
    pub algorithm: Fifo,
    pub current: Mutex<Option<Task>>,
    pub idle: Mutex<Option<nix::libc::ucontext_t>>,
}

impl Scheduler {
    /// Register a task to run
    pub fn create_task<F: Fn()>(&self, func: F) {
        eprintln!("Task created");
        // Should only register the task, not run it.
        // When add to queue
        let lock = *self.current.lock().unwrap();
        if let Some(Task::Ready(job)) = lock {
            self.algorithm
                .add_to_queue(&self.queue, Task::Ready(job.set_job_context(func)));
        } else {
            let job = Job::create_context();
            self.idle.lock().unwrap().get_or_insert(job.context);
            self.current.lock().unwrap().get_or_insert(Task::Ready(job));
            self.algorithm
                .add_to_queue(&self.queue, Task::Ready(job.set_job_context(func)));
        }
    }
    /// Run a given task. Not to be called directly by end-user.
    pub fn run_task(&self) {
        self.destroy_task();
    }

    /// Yield: go back to scheduler to run one of the ready tasks.
    pub fn yield_task(&self) {
        eprintln!("Calling yield");
        std::thread::sleep(std::time::Duration::from_secs(1));
        // check if current, yes = running task -> ready ELSE ready/new to running

        let current = *self.current.lock().unwrap();
        let next = self.algorithm.get_next_task(&self.queue);

        if let Some(Task::Ready(mut _next)) = next {
            if let Some(Task::Ready(mut _current)) = &current {
                if !_current.eq(&_next) {
                    self.algorithm.add_to_queue(&self.queue, current.unwrap());
                    _current.swap_context(&mut _next.context)
                }
            }
        } else if let Some(Task::Ready(mut _current)) = &current {
            let mut idle = self.idle.lock().unwrap().unwrap();
            _current.swap_context(&mut idle);
        }
    }
    /// Destroy at end of task. Not to be called directly.
    fn destroy_task(&self) {
        println!("Terminating task");
    }

    // Task::Blocked
    pub fn lock(&self) {}
    // Task::Ready
    pub fn unlock(&self) {}
}
