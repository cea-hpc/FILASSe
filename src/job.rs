use crossbeam_utils::thread;
use once_cell::sync::Lazy;
use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;

#[derive(Debug)]
/// E.g. struct for the FIFO scheduling algorithm
pub struct Fifo {
    /// number of processor
    pub queue: Mutex<Lazy<HashMap<QueueKind, VecDeque<Thread>>>>,
    pub virtual_processor: u32,
}

#[derive(Hash, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Thread {
    pub counter: u64,
    pub task: Task,
}

pub trait ThreadTrait {
    fn work(&mut self);
    fn create(&mut self, sched: &mut Box<dyn SchedulingAlgorithm>);
    fn exit(&mut self, sched: &mut Box<dyn SchedulingAlgorithm>);
}

impl ThreadTrait for Thread {
    fn work(&mut self) {
        self.counter += 1;
    }
    fn create(&mut self, _sched: &mut Box<dyn SchedulingAlgorithm>) {
        thread::scope(|s| {
            s.spawn(|_| {
                if let Task::Ready(id, run) = self.task {
                    self.task = Task::Running(id, run);
                    dbg!(id.id);
                }
            });
        })
        .unwrap();
        if self.counter < 10 {
            // Not working
            let backoff = crossbeam_utils::Backoff::new();
            while !backoff.is_completed() {
                backoff.snooze();
            }
            self.counter += 1;
        }
        self.exit(_sched)
    }
    fn exit(&mut self, sched: &mut Box<dyn SchedulingAlgorithm>) {
        if let Task::Running(id, _) = self.task {
            self.task = Task::Terminated(id);
            self.work();
            dbg!(self);
            sched.run();
        } else {
            panic!("task not running");
        }
        // SchedulingAlgorithm::get_next_task();
        //call scheduler
        //return values
        //get next task
    }
}

type ProcessId = u64;
#[derive(Hash, Debug, Copy, Clone, Eq, PartialEq)]
pub struct TaskId {
    // ID of tje task
    pub id: ProcessId,
    // ID of the parent task
    pub parent_id: ProcessId,
    // if the parent task wait : true
    pub waited: bool,
}

#[derive(Hash, Debug, Copy, Clone, Eq, PartialEq)]
pub struct RunnableTask {
    // For Shortest job next algorithm
    pub duration: u64,
    // For priority algorithm
    pub priority: u64,
    // Application to schedule
    pub function: fn(),
}

#[derive(Hash, Debug, Copy, Clone, Eq, PartialEq)]
pub enum Task {
    // Creation State
    New(TaskId, RunnableTask),
    // In Queue State, ready to be scheduled
    Ready(TaskId, RunnableTask),
    // In run
    Running(TaskId, RunnableTask),
    // Blocked by I/O
    Blocked(TaskId, RunnableTask),
    // Father dead
    Zombie(TaskId),
    // Before to be free
    Terminated(TaskId),
}

#[derive(Hash, Debug, Copy, Clone, Eq, PartialEq)]
/// Possible kinds of job queues
pub enum QueueKind {
    One,
    Ready,
    Blocked,
    Current,
}

pub static JOB_QUEUE: Mutex<Lazy<HashMap<QueueKind, VecDeque<Thread>>>> =
    Mutex::new(Lazy::new(|| HashMap::new()));

/// Trait that structs representing scheduling algorithms must implement
pub trait SchedulingAlgorithm {
    /// Specialize how each scheduling algorithm initializes its queues
    fn new() -> Self
    where
        Self: Sized;
    fn init_queues(queues: &mut HashMap<QueueKind, VecDeque<Thread>>)
    where
        Self: Sized,
    {
        queues.insert(QueueKind::One, VecDeque::new());
        queues.insert(QueueKind::Current, VecDeque::new());
    }
    fn create(&mut self, job: Task)
    where
        Self: Sized;
    fn run(&mut self);
    fn get_current(&mut self) -> Option<Thread>;

    fn set_current(&mut self, task: Option<Thread>);

    fn get_next_task(&mut self) -> Option<Thread>;

    fn push_task(&mut self, task: Thread);
    fn to_ready(&mut self, task: Task) -> Task {
        match task {
            Task::Running(id, run) => Task::Ready(id, run),
            Task::Blocked(id, run) => Task::Ready(id, run),
            _ => panic!("bad conversion to_ready"),
        }
    }
    fn to_running(task: Task) -> Task
    where
        Self: Sized,
    {
        match task {
            Task::Ready(id, run) => Task::Running(id, run),
            _ => panic!("bad convertion to_running"),
        }
    }
    fn to_blocked(&mut self, task: Task) -> Task {
        match task {
            Task::Running(id, run) => Task::Blocked(id, run),
            _ => {
                panic!("bad convertion to_blocked")
            }
        }
    }
    /// change state from Task::running to Task::Zombie
    fn to_zombie(&mut self, task: Task) -> Task {
        match task {
            Task::Running(id, _) => Task::Zombie(id),
            _ => panic!("Bad conversion"),
        }
    }
    fn to_terminated(&mut self, task: Task) -> Task {
        match task {
            Task::Running(id, _) => Task::Terminated(id),
            _ => panic!("Bad conversion"),
        }
    }
}

impl SchedulingAlgorithm for Fifo {
    fn new() -> Self {
        let fifo = Self {
            virtual_processor: 2,
            queue: Mutex::new(Lazy::new(|| HashMap::new())),
        };
        fifo.queue
            .lock()
            .unwrap()
            .insert(QueueKind::One, VecDeque::new());
        fifo.queue
            .lock()
            .unwrap()
            .insert(QueueKind::Current, VecDeque::new());
        fifo
    }
    fn set_current(&mut self, task: Option<Thread>) {
        if let Some(_task) = task {
            if let Ok(mut lock) = self.queue.lock() {
                if let Some(queue) = lock.get_mut(&QueueKind::Current) {
                    queue.push_front(_task);
                }
            }
        }
    }
    fn push_task(&mut self, task: Thread)
    where
        Self: Sized,
    {
        if let Some(queue) = self.queue.lock().unwrap().get_mut(&QueueKind::One) {
            queue.push_back(task);
        }
    }
    fn get_current(&mut self) -> Option<Thread> {
        match self.queue.lock() {
            Ok(mut lock) => match lock.get_mut(&QueueKind::Current) {
                Some(task) => task.pop_front(),
                None => panic!("no task"),
            },
            Err(_) => panic!("Poisoned mutex"),
        }
    }
    fn create(&mut self, task: Task) {
        if let Task::New(a, b) = task {
            self.push_task(Thread {
                counter: 0,
                task: Task::Ready(a, b),
            });
        }
    }

    fn get_next_task(&mut self) -> Option<Thread> {
        if let Some(task) = self
            .queue
            .lock()
            .expect("Failed to take lock")
            .get_mut(&QueueKind::One)
        {
            task.pop_front()
        } else {
            None
        }
    }
    /// Yield for preempt after q time
    fn run(&mut self) {
        if let Some(_task) = self.get_current() {
            // (run.function)();
            if let Some(mut next_task) = self.get_next_task() {
                // Schedule task
                // Set current / run the new context
                self.set_current(Some(next_task));
                // End task || Yield || Blocked

                next_task.create(self);
                // END
                // Some(Task::Terminated(id));
                // Yieldmut
                // yield()
                // push_task(Task::Ready(id, run));

                // blocked
                // push_task(Task::Blocked(id, run));
                // self.run();
            }
        } else {
            // Set current / run the new context
            if let Some(mut next) = self.get_next_task() {
                self.set_current(Some(next));
                next.create(self);
            }

            // self.run();
        }
    }
}

#[derive(Debug)]
pub struct Shortest {
    /// number of processor
    pub virtual_processor: u32,
    pub queue: Mutex<Lazy<HashMap<QueueKind, VecDeque<Thread>>>>,
}

impl SchedulingAlgorithm for Shortest {
    fn set_current(&mut self, task: Option<Thread>) {
        if let Some(_task) = task {
            if let Ok(mut lock) = self.queue.lock() {
                if let Some(queue) = lock.get_mut(&QueueKind::Current) {
                    queue.push_front(_task);
                }
            }
        }
    }
    fn new() -> Self {
        let fifo = Self {
            virtual_processor: 2,
            queue: Mutex::new(Lazy::new(|| HashMap::new())),
        };
        fifo.queue
            .lock()
            .unwrap()
            .insert(QueueKind::One, VecDeque::new());
        fifo.queue
            .lock()
            .unwrap()
            .insert(QueueKind::Current, VecDeque::new());
        fifo
    }
    fn create(&mut self, task: Task) {
        if let Task::New(a, b) = task {
            self.push_task(Thread {
                counter: 0,
                task: Task::Ready(a, b),
            });
        }
    }

    fn get_current(&mut self) -> Option<Thread> {
        match self.queue.lock() {
            Ok(mut lock) => match lock.get_mut(&QueueKind::Current) {
                Some(task) => task.pop_front(),
                None => panic!("no task"),
            },
            Err(_) => panic!("Poisoned mutex"),
        }
    }
    fn push_task(&mut self, task: Thread)
    where
        Self: Sized,
    {
        if let Some(queue) = self.queue.lock().unwrap().get_mut(&QueueKind::One) {
            queue.push_back(task);
        }
    }
    fn get_next_task(&mut self) -> Option<Thread> {
        if let Some(task) = self
            .queue
            .lock()
            .expect("Failed to take lock")
            .get_mut(&QueueKind::One)
        {
            if let Some(idx) = task.iter().enumerate().min_by_key(|(_, x)| match x.task {
                Task::Ready(_, a) => a.duration,
                _ => 2000,
            }) {
                task.remove(idx.0)
            } else {
                None
            }
        } else {
            None
        }
    }
    fn run(&mut self) {
        // Min(duration) is the next schedule
        if let Some(_task) = self.get_current() {
            // (run.function)();
            if let Some(mut next_task) = self.get_next_task() {
                // Schedule task
                // Set current / run the new context
                self.set_current(Some(next_task));
                next_task.create(self);
                // Some(Task::Running(id, run));
                // End task || Yield || Blocked

                // END
                // Some(Task::Terminated(id));
                // Yield
                // yield()
                // push_task(Task::Ready(id, run));

                // blocked
                // push_task(Task::Blocked(id, run));
                // self.run();
                // next_task.create();
            }
        } else {
            // Set current / run the new context
            if let Some(mut next) = self.get_next_task() {
                self.set_current(Some(next));
                next.create(self);
            }

            // self.run();
        }
    }
}
