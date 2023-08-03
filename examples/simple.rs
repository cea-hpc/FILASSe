use std::{collections::VecDeque, sync::Mutex};

/// Dummy struct for Scheduler
/// Even not enough to properly run a real theaded program as a task queue should be necessary
struct Scheduler {
    queue: Mutex<VecDeque<Task>>,
    algorithm: Fifo,
}

trait Algorithm {
    fn get_next_task(&self, queue: &Mutex<VecDeque<Task>>) -> Option<Task>;
}

struct Fifo {}
impl Algorithm for Fifo {
    fn get_next_task(&self, queue: &Mutex<VecDeque<Task>>) -> Option<Task> {
        let mut guard = queue.lock().unwrap();
        guard.pop_front()
    }
}

pub struct SchedulerUser {
    scheduler: Scheduler,
}

struct Job {
    func: Box<dyn Fn() -> ()>,
}

enum Task {
    /// Creation State
    New(Job),
    // In Queue State, ready to be scheduled
    Ready(Job),
}

impl SchedulerUser {
    pub fn create_task<F: Fn() -> () + 'static>(&self, my_func: F) {
        self.scheduler.create_task(Task::New(Job {
            func: Box::new(my_func),
        }));
    }

    pub fn yield_task(&self) {
        self.scheduler.yield_task();
    }
}

impl Scheduler {
    /// Register a task to run
    fn create_task(&self, task: Task) {
        eprintln!("Task created");
        // Should only register the task, not run it.
        // When add to queue
        if let Task::New(job) = task {
            self.queue.lock().unwrap().push_back(Task::Ready(job));
        }
        self.yield_task();
    }

    /// Yield: go back to scheduler to run one of the ready tasks.
    fn yield_task(&self) {
        eprintln!("Calling yield");

        if let Some(_retrun) = self.algorithm.get_next_task(&self.queue) {
            if let Task::Ready(job) = _retrun {
                (job.func)();
            } else {
                eprintln!("Task is not ready");
            }
        } else {
            eprintln!("Nothing to do, returning to previous execution");
        }
    }
}

pub static SCHEDULER: SchedulerUser = SchedulerUser {
    scheduler: Scheduler {
        queue: Mutex::new(VecDeque::new()),
        algorithm: Fifo {},
        // current: None,
    },
};

unsafe impl Sync for Task {}
unsafe impl Send for Task {}

fn task1() -> () {
    eprintln!("Starting task 1 execution");
    // Make scheduler able to schedule another ready task
    // Yield can also be hidden into a lock
    SCHEDULER.yield_task();

    eprintln!("Resuming task 1");
    // Creating a child task
    SCHEDULER.create_task(task2);

    eprintln!("Resuming task 1");
    // Make scheduler able to schedule another ready task
    SCHEDULER.yield_task();

    eprintln!("Ending task 1");
}

fn task2() -> () {
    eprintln!("Starting task 2");
    SCHEDULER.yield_task();

    eprintln!("Resuming task 2");
    SCHEDULER.create_task(|| {
        eprintln!("Executing a sub task");
    });

    eprintln!("Ending task 2");
}

fn main() {
    SCHEDULER.create_task(task1);
}
