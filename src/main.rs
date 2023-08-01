use filasse::job::*;

pub fn job() {
    dbg!(1 + 1);
}
// pub fn sleep_job() {
//     thread::sleep(Duration::from_secs(1));
//     dbg!(thread::current());
// }

fn main() {
    let fifo = Fifo::new();
    let mut _sched = Shortest::new();
    Shortest::init_queues(&mut JOB_QUEUE.lock().unwrap());
    // let mut  = sched;
    // let b = round_robin();
    // Scheduler::fthread_create(job, false);
    let task = Task::New(
        TaskId {
            id: 0,
            parent_id: 0,
            waited: false,
        },
        RunnableTask {
            function: job,
            duration: 100,
            priority: 1,
        },
    );
    let task1 = Task::New(
        TaskId {
            id: 1,
            parent_id: 0,
            waited: false,
        },
        RunnableTask {
            function: job,
            duration: 150,
            priority: 1,
        },
    );
    let task2 = Task::New(
        TaskId {
            id: 2,
            parent_id: 0,
            waited: false,
        },
        RunnableTask {
            function: job,
            duration: 50,
            priority: 1,
        },
    );
    _sched.create(task);
    _sched.create(task1);
    _sched.create(task2);
    dbg!(&_sched);
    _sched.run();
    // let mut a = Thread { counter: 0 };
    // a.call_scheduler(sched);
    dbg!(&_sched);
}
