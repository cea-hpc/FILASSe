pub trait State {}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct New {
    pub duration: u64,
    pub priority: u32,
}
impl State for New {}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ready {
    pub duration: u64,
    pub priority: u32,
}
impl State for Ready {}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Running {
    pub duration: u64,
    pub priority: u32,
}
impl State for Running {}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Blocked {
    pub duration: u64,
    pub priority: u32,
}
impl State for Blocked {}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Zombie {}
impl State for Zombie {}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Terminated {}
impl State for Terminated {}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Job<Status: State> {
    pub pid: u32,
    pub parent: u32,
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
