use crate::job::*;
use std::collections::VecDeque;
/// Struct of the scheduler
///
/// ```rust, ignore
///# use filasse::job::*;
///# use std::collections::VecDeque;
/// pub struct Scheduler {
///     queue: VecDeque<Job<Ready>>,
///     blocked: VecDeque<Job<Blocked>>,
///     zombie: VecDeque<Job<Zombie>>,
///     q: u64,
///     pid_count: u64,
///     available: bool,
/// }
///```

#[derive(Debug, Clone)]
pub struct Scheduler {
    queue: VecDeque<Job<Ready>>,
    blocked: VecDeque<Job<Blocked>>,
    zombie: VecDeque<Job<Zombie>>,
    q: u64,
    pid_count: u64,
    available: bool,
}

impl Default for Scheduler {
    fn default() -> Self {
        Scheduler {
            queue: VecDeque::<Job<Ready>>::new(),
            blocked: VecDeque::<Job<Blocked>>::new(),
            zombie: VecDeque::<Job<Zombie>>::new(),
            q: 100,
            pid_count: 1,
            available: true,
        }
    }
}

impl Scheduler {
    /// Initialisation method
    ///
    /// This method initialise the scheduler. It takes in argument the quantum of time desire. The method returns a Scheduler struct. There is also the default method which return the struct with a quantum fix at 100.
    ///
    /// # Examples :
    /// Example 1 :
    /// ```rust, ignore
    ///# use filasse::scheduler::*;
    /// let sched = Scheduler::new(200);
    ///```
    /// Example 2 :
    /// ```rust, ignore
    ///# use filasse::scheduler::*;
    /// let sched = Scheduler::default();
    ///````
    pub fn new(q: u64) -> Self {
        Scheduler {
            queue: VecDeque::<Job<Ready>>::new(),
            blocked: VecDeque::<Job<Blocked>>::new(),
            zombie: VecDeque::<Job<Zombie>>::new(),
            q,
            pid_count: 1,
            available: true,
        }
    }

    /// Getter quantum
    ///
    /// The method allows you to get the quantum's value
    ///
    /// # Example :
    /// ```rust, ignore
    ///# use filasse::scheduler::*;
    /// let sched = Scheduler::default();
    /// sched.quantum();
    ///```
    pub fn quantum(&self) -> u64 {
        self.q
    }

    /// Setter quantum
    ///
    /// The method allows you to set the quantum's value
    ///
    /// # Example :
    /// ```rust, ignore
    ///# use filasse::scheduler::*;
    /// let mut sched = Scheduler::default();
    /// sched.set_quantum(200);
    ///```
    pub fn set_quantum(&mut self, value: u64) {
        self.q = value;
    }

    /// TBD
    ///
    /// TBD
    ///
    /// # Example :
    /// ```rust, ignore
    ///# use filasse::scheduler::*;
    /// let sched = Scheduler::default();
    /// sched.is_available();
    ///```
    pub fn is_available(&self) -> bool {
        self.available
    }

    /// Getter pid_count
    ///
    /// The method allows you to get the pid_counter's value
    ///
    /// # Example :
    /// ```rust, ignore
    ///# use filasse::scheduler::*;
    /// let sched = Scheduler::default();
    /// sched.pid_count();
    ///```
    pub fn pid_count(&self) -> u64 {
        self.pid_count
    }

    /// Getter Queue
    ///
    /// The method allows you to get the queue vector.
    ///
    /// # Example :
    /// ```rust, ignore
    ///# use filasse::scheduler::*;
    /// let sched = Scheduler::default();
    /// sched.queue();
    ///```
    pub fn queue(&self) -> &VecDeque<Job<Ready>> {
        &self.queue
    }

    /// Getter Zombie
    ///
    /// The method allows you to get the zombie vector.
    ///
    /// # Example :
    /// ```rust, ignore
    ///# use filasse::scheduler::*;
    /// let sched = Scheduler::default();
    /// sched.zombie();
    ///```
    pub fn zombie(&self) -> &VecDeque<Job<Zombie>> {
        &self.zombie
    }

    /// Add to the scheduler
    ///
    /// The method takes in arguments a job. This job will be added in the queue.
    ///
    /// # Example :
    /// ```rust, ignore
    ///# use filasse::scheduler::*;
    ///# use filasse::job::*;
    /// let mut sched = Scheduler::default();
    /// let mut job = Job::default();
    /// sched.add_to_scheduler(&mut job);
    ///```
    pub fn add_to_scheduler(&mut self, job: &mut Job<New>) {
        let mut joba: Job<Ready> = Job::from(*job);
        joba.pid = self.pid_count;
        self.queue.push_back(joba);
        self.pid_count += 1;
    }

    pub fn lock(&mut self) {
        let job: Job<Running> = Job::from(self.queue.pop_front().unwrap());
        self.blocked.push_back(Job::from(job));
    }

    pub fn unlock(&mut self) {
        let job: Job<Ready> = Job::from(self.blocked.pop_front().unwrap());
        self.queue.push_back(job);
    }

    /// Process
    ///
    /// TBD
    ///
    /// # Example :
    /// ```rust, ignore
    ///# use filasse::scheduler::*;
    ///# use filasse::job::*;
    /// let mut sched = Scheduler::new(1);
    ///    let mut job = Job::new(1, 2, 2, 2);
    /// sched.add_to_scheduler(&mut job);
    /// sched.process();
    ///```
    pub fn process(&mut self) {
        let mut job: Job<Running> = Job::from(self.queue.pop_front().unwrap());
        if job.state.duration > 0 {
            if (job.state.duration - self.q) > 0 {
                job.state.duration -= self.q;
                let job = Job::from(job);
                self.queue.push_back(job);
            } else {
                job.state.duration = 0;
                let job = Job::from(job);
                self.zombie.push_back(job);
            }
        }
    }
}

/// Algorithm Round robin
///
/// TBD
///
/// # Example :
/// ```rust, ignore
///# use filasse::scheduler::*;
///# use filasse::job::*;
/// let mut sched = Scheduler::new(1);
/// let mut job = Job::new(1, 2, 2, 2);
/// sched.add_to_scheduler(&mut job);
/// sched.process();
/// round_robin(&mut sched);
///```
pub fn round_robin(sched: &mut Scheduler) {
    loop {
        if !sched.queue.is_empty() {
            match sched.queue.get(0) {
                Some(_) => {
                    println!("SUR LE PROCESSUS : {:?}", sched.queue[0]);
                    sched.process(); // pendant Q temps
                }
                None => break,
            }
            println!("Queue : {:?}\nZombie : {:?}\n", sched.queue(), sched.zombie);
        } else {
            println!("No Processus");
            break;
        }
    }
}
