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
