/// Dummy struct for Scheduler
/// Even not enough to properly run a real theaded program as a task queue should be necessary
struct Scheduler {}

impl Scheduler {
    /// Register a task to run
    /// Think `pthread_create` alike
    pub fn create_task(&self, my_func: impl Fn() -> ()) {
        println!("Task created");
        // Should only register the task, not run it.
        self.run_task(my_func);
    }

    /// Run a given task. Not to be called directly by end-user.
    fn run_task(&self, my_func: impl Fn() -> ()) {
        my_func();
        self.destroy_task();
    }

    /// Yield: go back to scheduler to run one of the ready tasks.
    pub fn yield_task(&self) {
        println!("Calling yield");
        // Run the next ready task!
    }

    /// Destroy at end of task. Not to be called directly.
    fn destroy_task(&self) {
        println!("Terminating task");
    }
}

static SCHEDULER: Scheduler = Scheduler {};

fn task1() -> () {
    println!("Executing task 1");
    // Make scheduler able to schedule another ready task
    // Yield can also be hidden into a lock
    SCHEDULER.yield_task();
    println!("Executing task 1");
    // Creating a child task
    SCHEDULER.create_task(task2);
    println!("Executing task 1");
    // Make scheduler able to schedule another ready task
    SCHEDULER.yield_task();
    println!("Executing task 1");
}

fn task2() -> () {
    println!("Executing task 2");
    SCHEDULER.yield_task();
    println!("Executing task 2");
    SCHEDULER.create_task(|| {
        println!("Executing a sub task");
    });
    println!("Executing task 2");
}

fn main() {
    SCHEDULER.create_task(task1);
}
