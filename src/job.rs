use crate::threads::*;
use nix::libc::ucontext_t;

type ProcessId = u64;

#[derive(Clone, Copy)]
pub struct JobId {
    pub id: ProcessId,
    pub parent_id: ProcessId,
}

#[derive(Clone, Copy)]
pub struct RunnableJob {
    pub duration: u64,
    pub priority: u32,
    pub context: ucontext_t,
}
#[derive(Clone, Copy)]
pub enum Job {
    New(JobId, RunnableJob),
    Ready(JobId, RunnableJob),
    Running(JobId, RunnableJob),
    Blocked(JobId, RunnableJob),
    Zombie(JobId),
    Terminated(JobId),
}

impl Job {
    pub fn new(parent_id: ProcessId, duration: u64, priority: u32, context: ucontext_t) -> Job {
        Job::New(
            JobId { id: 0, parent_id },
            RunnableJob {
                duration,
                priority,
                context,
            },
        )
    }

    pub fn ready(self) -> Job {
        match self {
            Job::New(id, run) | Job::Running(id, run) | Job::Blocked(id, run) => {
                Job::Ready(id, run)
            }
            _ => panic!("Bad Job's state"),
        }
    }

    pub fn running(self) -> Job {
        match self {
            Job::Ready(id, run) => Job::Running(id, run),
            _ => panic!(),
        }
    }

    pub fn blocked(self) -> Job {
        match self {
            Job::Running(id, run) => Job::Blocked(id, run),
            _ => panic!(),
        }
    }

    pub fn zombie(self) -> Job {
        match self {
            Job::Running(id, _) => Job::Zombie(id),
            _ => panic!(),
        }
    }
    pub fn terminated(self) -> Job {
        match self {
            Job::Zombie(id) => Job::Terminated(id),
            _ => panic!(),
        }
    }
}

// /// Trait State
// ///
// /// This trait allows you to obtain only the right States from the automaton.
// ///
// /// The states : `New, Ready, Running, Bloked, Zombie, Terminated`
// pub trait State {}

// /// State New
// ///
// /// The new state is optainable at the creation of the job.
// ///
// ///
// #[derive(Debug, Copy, Clone, PartialEq)]
// pub struct New {
//     pub context: ucontext_t,
//     pub duration: u64,
//     pub priority: u32,
// }
// impl State for New {}

// /// State Ready
// ///
// /// The ready state is optainable at the creation of the job.
// ///
// ///
// #[derive(Debug, Copy, Clone, PartialEq)]
// pub struct Ready {
//     pub context: ucontext_t,
//     pub duration: u64,
//     pub priority: u32,
// }
// impl State for Ready {}

// /// State Running
// ///
// /// The running state is optainable at the creation of the job.
// ///
// ///
// #[derive(Debug, Copy, Clone, PartialEq)]
// pub struct Running {
//     pub context: ucontext_t,
//     pub duration: u64,
//     pub priority: u32,
// }
// impl State for Running {}

// /// State Blocked
// ///
// /// The blocked state is optainable at the creation of the job.
// ///
// ///
// #[derive(Debug, Copy, Clone, PartialEq)]
// pub struct Blocked {
//     pub context: ucontext_t,
//     pub duration: u64,
//     pub priority: u32,
// }
// impl State for Blocked {}

// /// State Zombie
// ///
// /// The zombie state is optainable at the creation of the job.
// ///
// ///
// #[derive(Debug, Copy, Clone, PartialEq)]
// pub struct Zombie {}
// impl State for Zombie {}

// /// State Terminated
// ///
// /// The terminated state is optainable at the creation of the job.
// ///
// ///
// #[derive(Debug, Copy, Clone, PartialEq)]
// pub struct Terminated {}
// impl State for Terminated {}

// /// Job
// ///
// /// Pid, parent pid, state
// #[derive(Debug, Copy, Clone, PartialEq)]
// pub struct Job<Status: State> {
//     pub pid: u64,
//     pub parent: u64,
//     pub state: Status,
// }

// /// Job Creation
// ///
// /// # Example :
// /// ```rust, ignore
// /// # use filasse::job::*;
// /// let foo : Job<New>= Job::new(5, 2, 2, 1);
// /// # assert!(foo.state.duration == 2);
// /// ```
// impl Job<New> {
//     pub fn new(context: ucontext_t, pid: u64, parent: u64, duration: u64, priority: u32) -> Self {
//         Self {
//             pid,
//             parent,
//             state: New {
//                 duration,
//                 priority,
//                 context,
//             },
//         }
//     }
// }

// /// Job Default
// ///
// /// # Example :
// /// ```rust, ignore
// /// # use filasse::job::*;
// /// let foo : Job<New>= Job::default();
// /// # assert!(foo.state == New{duration: 0, priority: 0});
// /// ```
// impl Default for Job<New> {
//     fn default() -> Self {
//         Self {
//             pid: 0,
//             parent: 0,
//             state: New {
//                 context: create_ctx(),
//                 duration: 0,
//                 priority: 0,
//             },
//         }
//     }
// }

// impl From<Job<Zombie>> for Job<Terminated> {
//     fn from(prev: Job<Zombie>) -> Job<Terminated> {
//         Job {
//             pid: prev.pid,
//             parent: prev.parent,
//             state: Terminated {},
//         }
//     }
// }

// impl From<Job<Running>> for Job<Zombie> {
//     fn from(prev: Job<Running>) -> Job<Zombie> {
//         Job {
//             pid: prev.pid,
//             parent: prev.parent,
//             state: Zombie {},
//         }
//     }
// }

// impl From<Job<Blocked>> for Job<Ready> {
//     fn from(prev: Job<Blocked>) -> Job<Ready> {
//         Job {
//             pid: prev.pid,
//             parent: prev.parent,
//             state: Ready {
//                 context: prev.state.context,
//                 duration: prev.state.duration,
//                 priority: prev.state.priority,
//             },
//         }
//     }
// }

// impl From<Job<Running>> for Job<Ready> {
//     fn from(prev: Job<Running>) -> Job<Ready> {
//         Job {
//             pid: prev.pid,
//             parent: prev.parent,
//             state: Ready {
//                 context: prev.state.context,
//                 duration: prev.state.duration,
//                 priority: prev.state.priority,
//             },
//         }
//     }
// }

// impl From<Job<Running>> for Job<Blocked> {
//     fn from(prev: Job<Running>) -> Job<Blocked> {
//         Job {
//             pid: prev.pid,
//             parent: prev.parent,
//             state: Blocked {
//                 context: prev.state.context,
//                 duration: prev.state.duration,
//                 priority: prev.state.priority,
//             },
//         }
//     }
// }

// /// Job conversion from Ready to Running
// ///
// /// # Example :
// /// ```rust, ignore
// /// # use filasse::job::*;
// ///# let foo : Job<New>= Job::new(5, 2, 2, 1);
// ///# let bar : Job<Ready> = Job::from(foo);
// /// let foo: Job<Running> = Job::from(bar);
// /// # assert!(foo.state == Running{duration: 2, priority: 1});
// /// ```
// impl From<Job<Ready>> for Job<Running> {
//     fn from(prev: Job<Ready>) -> Job<Running> {
//         Job {
//             pid: prev.pid,
//             parent: prev.parent,
//             state: Running {
//                 context: prev.state.context,
//                 duration: prev.state.duration,
//                 priority: prev.state.priority,
//             },
//         }
//     }
// }

// /// Job conversion from New to Reay
// ///
// /// # Example :
// /// ```rust, ignore
// /// # use filasse::job::*;
// /// let foo : Job<New>= Job::new(5, 2, 2, 1);
// /// let bar : Job<Ready> = Job::from(foo);
// /// # assert!(bar.state == Ready{duration: 2, priority: 1});
// /// ```
// impl From<Job<New>> for Job<Ready> {
//     fn from(prev: Job<New>) -> Job<Ready> {
//         Job {
//             pid: prev.pid,
//             parent: prev.parent,
//             state: Ready {
//                 context: prev.state.context,
//                 duration: prev.state.duration,
//                 priority: prev.state.priority,
//             },
//         }
//     }
// }
