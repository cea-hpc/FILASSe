use filasse::{self, filasse::SCHEDULER};
/// Dummy struct for Scheduler
/// Even not enough to properly run a real theaded program as a task queue should be necessary
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

fn task3() -> () {
    eprintln!("Executing task 3");
}

fn main() {
    SCHEDULER.create_task(task1);
    // SCHEDULER.create_task(task3);
    SCHEDULER.join();
    eprintln!("end of program");
}
