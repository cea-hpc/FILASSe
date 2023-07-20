use once_cell::sync::Lazy;
use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;

// #[derive(Hash, Debug, Copy, Clone, Eq, PartialEq)]
// /// Possible algorithms
// pub enum Scheduler {
//     /// Fifo, First in first out, Coopérative algorithm
//     Fifo,
//     /// Round Robin, preemptive algorithm
//     Roundrobin,
//     /// Shortest Job Next, duration algorithm
//     Shortest,
// }

#[derive(Hash, Debug, Copy, Clone, Eq, PartialEq)]
pub struct RoundRobin {
    /// number of processor
    virtual_processor: u32,
    /// execution time for each task before yield
    quantum: u64,
}

#[derive(Hash, Debug, Copy, Clone, Eq, PartialEq)]
/// E.g. struct for the FIFO scheduling algorithm
pub struct Fifo {
    /// number of processor
    pub virtual_processor: u32,
}

#[derive(Hash, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Shortest {
    /// number of processor
    virtual_processor: u32,
    duration: u32,
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

pub static JOB_QUEUE: Mutex<Lazy<HashMap<QueueKind, VecDeque<Task>>>> =
    Mutex::new(Lazy::new(|| HashMap::new()));

/// Trait that structs representing scheduling algorithms must implement
pub trait SchedulingAlgorithm {
    /// Specialize how each scheduling algorithm initializes its queues
    fn init_queues(queues: &mut HashMap<QueueKind, VecDeque<Task>>);
    fn run(&self) -> Option<Task>;

    fn get_next_task() -> Option<Task> {
        let mut task = JOB_QUEUE.lock().expect("Failed to take lock");
        // task.pop_front()
        None
    }
    fn push_task(task: Task) {
        if let Task::New(id, run) = task {
            if let Some(mut queue) = JOB_QUEUE.lock().unwrap().get_mut(&QueueKind::One) {
                queue.push_back(Task::Ready(id, run));
            }
        }
    }
    fn to_ready(&mut self) -> Option<Task> {
        match Self::get_next_task() {
            Some(Task::Running(id, run)) => Some(Task::Ready(id, run)),
            Some(Task::Blocked(id, run)) => Some(Task::Ready(id, run)),
            _ => None,
        }
    }
    fn to_running(&mut self) -> Option<Task> {
        match Self::get_next_task() {
            Some(Task::Ready(id, run)) => Some(Task::Running(id, run)),
            _ => None,
        }
    }
    fn to_blocked(&mut self) -> Task {
        match Self::get_next_task() {
            Some(Task::Running(id, run)) => Task::Blocked(id, run),
            _ => {
                panic!("To_blocked")
            }
        }
    }
    /// change state from Task::running to Task::Zombie
    fn to_zombie(&mut self) {
        match Self::get_next_task() {
            _ => {}
        }
    }
    fn to_terminated(&mut self) {
        match Self::get_next_task() {
            _ => {}
        }
    }
}

impl SchedulingAlgorithm for RoundRobin {
    fn init_queues(queues: &mut HashMap<QueueKind, VecDeque<Task>>) {
        queues.insert(QueueKind::Ready, VecDeque::new());
        queues.insert(QueueKind::Blocked, VecDeque::new());
    }
    fn run(&self) -> Option<Task> {
        let quantum = &self.quantum;
        // Yield after quantum time;
        if let Some(Task::Ready(id, run)) = Self::get_next_task() {
            // Schedule task
            // Some(Task::Running(id, run));
            // End task || Yield || Blocked

            // END
            // Some(Task::Terminated(id));
            // Yield
            // yield()
            // push_task(Task::Ready(id, run));

            // blocked
            // push_task(Task::Blocked(id, run));
            None
        } else {
            None
        }
    }
}

impl SchedulingAlgorithm for Fifo {
    fn init_queues(queues: &mut HashMap<QueueKind, VecDeque<Task>>) {
        // insert necessary queues for a FIFO scheduling algorithm...
        queues.insert(QueueKind::One, VecDeque::new());
    }
    fn run(&self) -> Option<Task> {
        if let Some(Task::Ready(id, run)) = Self::get_next_task() {
            // Schedule task
            // Some(Task::Running(id, run));
            // End task || Yield || Blocked

            // END
            // Some(Task::Terminated(id));
            // Yield
            // yield()
            // push_task(Task::Ready(id, run));

            // blocked
            // push_task(Task::Blocked(id, run));
            None
        } else {
            None
        }
    }
}
