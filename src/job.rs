/// Trait State
///
/// This trait allows you to obtain only the right States from the automaton.
///
/// The states : `New, Ready, Running, Bloked, Zombie, Terminated`
pub trait State {}

/// State New
///
/// The new state is optainable at the creation of the job.
///
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct New {
    pub duration: u64,
    pub priority: u32,
}
impl State for New {}

/// State Ready
///
/// The ready state is optainable at the creation of the job.
///
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ready {
    pub duration: u64,
    pub priority: u32,
}
impl State for Ready {}

/// State Running
///
/// The running state is optainable at the creation of the job.
///
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Running {
    pub duration: u64,
    pub priority: u32,
}
impl State for Running {}

/// State Blocked
///
/// The blocked state is optainable at the creation of the job.
///
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Blocked {
    pub duration: u64,
    pub priority: u32,
}
impl State for Blocked {}

/// State Zombie
///
/// The zombie state is optainable at the creation of the job.
///
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Zombie {}
impl State for Zombie {}

/// State Terminated
///
/// The terminated state is optainable at the creation of the job.
///
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Terminated {}
impl State for Terminated {}

/// Job Creation
///
/// # Example :
/// ```rust, ignore
/// # use filasse::job::*;
/// let foo : Job<New>= Job::new(5, 2, 2, 1);
/// # assert!(foo.state.duration == 2);
/// ```
impl Job<New> {
    pub fn new(pid: u64, parent: u64, duration: u64, priority: u32) -> Self {
        Self {
            pid,
            parent,
            state: New { duration, priority },
        }
    }
}

/// Job Default
///
/// # Example :
/// ```rust, ignore
/// # use filasse::job::*;
/// let foo : Job<New>= Job::default();
/// # assert!(foo.state == New{duration: 0, priority: 0});
/// ```
impl Default for Job<New> {
    fn default() -> Self {
        Self {
            pid: 0,
            parent: 0,
            state: New {
                duration: 0,
                priority: 0,
            },
        }
    }
}

/// Job
///
/// Pid, parent pid, state
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Job<Status: State> {
    pub pid: u64,
    pub parent: u64,
    pub state: Status,
}

impl From<Job<Zombie>> for Job<Terminated> {
    fn from(prev: Job<Zombie>) -> Job<Terminated> {
        Job {
            pid: prev.pid,
            parent: prev.parent,
            state: Terminated {},
        }
    }
}

impl From<Job<Running>> for Job<Zombie> {
    fn from(prev: Job<Running>) -> Job<Zombie> {
        Job {
            pid: prev.pid,
            parent: prev.parent,
            state: Zombie {},
        }
    }
}

impl From<Job<Blocked>> for Job<Ready> {
    fn from(prev: Job<Blocked>) -> Job<Ready> {
        Job {
            pid: prev.pid,
            parent: prev.parent,
            state: Ready {
                duration: prev.state.duration,
                priority: prev.state.priority,
            },
        }
    }
}

impl From<Job<Running>> for Job<Ready> {
    fn from(prev: Job<Running>) -> Job<Ready> {
        Job {
            pid: prev.pid,
            parent: prev.parent,
            state: Ready {
                duration: prev.state.duration,
                priority: prev.state.priority,
            },
        }
    }
}

impl From<Job<Running>> for Job<Blocked> {
    fn from(prev: Job<Running>) -> Job<Blocked> {
        Job {
            pid: prev.pid,
            parent: prev.parent,
            state: Blocked {
                duration: prev.state.duration,
                priority: prev.state.priority,
            },
        }
    }
}

/// Job conversion from Ready to Running
///
/// # Example :
/// ```rust, ignore
/// # use filasse::job::*;
///# let foo : Job<New>= Job::new(5, 2, 2, 1);
///# let bar : Job<Ready> = Job::from(foo);
/// let foo: Job<Running> = Job::from(bar);
/// # assert!(foo.state == Running{duration: 2, priority: 1});
/// ```
impl From<Job<Ready>> for Job<Running> {
    fn from(prev: Job<Ready>) -> Job<Running> {
        Job {
            pid: prev.pid,
            parent: prev.parent,
            state: Running {
                duration: prev.state.duration,
                priority: prev.state.priority,
            },
        }
    }
}

/// Job conversion from New to Reay
///
/// # Example :
/// ```rust, ignore
/// # use filasse::job::*;
/// let foo : Job<New>= Job::new(5, 2, 2, 1);
/// let bar : Job<Ready> = Job::from(foo);
/// # assert!(bar.state == Ready{duration: 2, priority: 1});
/// ```
impl From<Job<New>> for Job<Ready> {
    fn from(prev: Job<New>) -> Job<Ready> {
        Job {
            pid: prev.pid,
            parent: prev.parent,
            state: Ready {
                duration: prev.state.duration,
                priority: prev.state.priority,
            },
        }
    }
}
