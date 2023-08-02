use std::{collections::VecDeque, sync::Mutex};

use once_cell::sync::Lazy;

/// Dummy struct for Scheduler
/// Even not enough to properly run a real theaded program as a task queue should be necessary
struct Scheduler {
    queue: VecDeque<Task>,
    algorithm: Fifo,
}

trait Algorithm {
    fn get_next_task(&self, queue: &mut VecDeque<Task>) -> Option<Task>;
}

struct Fifo {}
impl Algorithm for Fifo {
    fn get_next_task(&self, queue: &mut VecDeque<Task>) -> Option<Task> {
        queue.pop_front()
    }
}

pub struct SchedulerUser {
    scheduler: Scheduler,
}

struct Job {
    func: Box<dyn Fn() -> ()>,
}
enum Task {
    // Creation State
    New(Job),
    // In Queue State, ready to be scheduled
    Ready(Job),
    // In run
    Running(Job),
    // Blocked by I/O
    Blocked(Job),
    // Father dead
    Zombie(Job),
    // Before to be free
    Terminated(Job),
}

impl SchedulerUser {
    pub fn create_task<F: Fn() -> () + 'static>(&mut self, my_func: F) {
        self.scheduler.create_task(Task::New(Job {
            func: Box::new(my_func),
        }));
    }
    pub fn yield_task(&mut self) {
        dbg!("blocked ?");
        self.scheduler.yield_task();
        // Run the next ready task!
    }
    fn set_algorithm(_algorithm: impl Algorithm) {}
}

impl Scheduler {
    /// Register a task to run
    /// Think `pthread_create` alike
    fn create_task(&mut self, task: Task) {
        println!("Task created");
        // Should only register the task, not run it.
        // When add to queue
        if let Task::New(job) = task {
            // (job.func)();
            self.queue.push_back(Task::Ready(job));
        }
        // self.run_task(task);
        self.yield_task();
    }

    /// Run a given task. Not to be called directly by end-user.
    fn run_task(&self, task: Task) {
        match task {
            Task::Ready(job) => (job.func)(),
            _ => (),
        }
        self.destroy_task();
    }

    /// Yield: go back to scheduler to run one of the ready tasks.
    fn yield_task(&mut self) {
        println!("Calling yield");
        if let Some(_retrun) = self.algorithm.get_next_task(&mut self.queue) {
            if let Task::Ready(job) = _retrun {
                (job.func)();
            } else {
                dbg!("bhb");
            }
        } else {
            dbg!("aha");
        }
        // Run the next ready task!
    }

    /// Destroy at end of task. Not to be called directly.
    fn destroy_task(&self) {
        println!("Terminating task");
    }
}

pub static SCHEDULER: Mutex<SchedulerUser> = Mutex::new(SchedulerUser {
    scheduler: Scheduler {
        queue: VecDeque::new(),
        algorithm: Fifo {},
    },
});

// static QUEUE: Mutex<Lazy<VecDeque<Task>>> = Mutex::new(Lazy::new(|| VecDeque::new()));

unsafe impl Sync for Task {}
unsafe impl Send for Task {}

fn task1() -> () {
    println!("Executing task 1");
    // Make scheduler able to schedule another ready task
    // Yield can also be hidden into a lock
    SCHEDULER.lock().unwrap().yield_task();
    println!("Executing task 1");
    // Creating a child task
    SCHEDULER.lock().unwrap().create_task(task2);
    println!("Executing task 1");
    // Make scheduler able to schedule another ready task
    SCHEDULER.lock().unwrap().yield_task();
    println!("Executing task 1");
}

fn task2() -> () {
    println!("Executing task 2");
    SCHEDULER.lock().unwrap().yield_task();
    println!("Executing task 2");
    SCHEDULER.lock().unwrap().create_task(|| {
        println!("Executing a sub task");
    });
    println!("Executing task 2");
}

fn main() {
    SCHEDULER.lock().unwrap().create_task(task1);
}
