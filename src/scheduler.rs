use crate::job::*;
use std::collections::HashMap;

/// Struct of the scheduler
///
///```
///# use filasse::job::*;
///# use std::collections::HashMap;
/// pub struct Scheduler {
///    queue: Vec<Job<Ready>>,
///    blocked: Vec<Job<Blocked>>,
///    zombie: Vec<Job<Zombie>>,
///     q: u64,
///     pid_count: u64,
///     available: bool,
/// }
///```

#[derive(Debug, Clone)]
pub struct Scheduler {
    // processus: HashMap<u64, Job<Ready>>,
    queue: Vec<Job<Ready>>,
    blocked: Vec<Job<Blocked>>,
    zombie: Vec<Job<Zombie>>,
    q: u64,
    pid_count: u64,
    available: bool,
}

impl Default for Scheduler {
    fn default() -> Self {
        Scheduler {
            queue: Vec::<Job<Ready>>::new(),
            blocked: Vec::<Job<Blocked>>::new(),
            zombie: Vec::<Job<Zombie>>::new(),
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
    /// ```
    ///# use filasse::scheduler::*;
    /// let sched = Scheduler::new(200);
    ///```
    /// Example 2 :
    ///```
    ///# use filasse::scheduler::*;
    /// let sched = Scheduler::default();
    ///````
    pub fn new(q: u64) -> Self {
        Scheduler {
            queue: Vec::<Job<Ready>>::new(),
            blocked: Vec::<Job<Blocked>>::new(),
            zombie: Vec::<Job<Zombie>>::new(),
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
    /// ```
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
    /// ```
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
    /// ```
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
    /// ```
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
    /// ```
    ///# use filasse::scheduler::*;
    /// let sched = Scheduler::default();
    /// sched.queue();
    ///```
    pub fn queue(&self) -> &Vec<Job<Ready>> {
        &self.queue
    }

    /// Getter Zombie
    ///
    /// The method allows you to get the zombie vector.
    ///
    /// # Example :
    /// ```
    ///# use filasse::scheduler::*;
    /// let sched = Scheduler::default();
    /// sched.zombie();
    ///```
    pub fn zombie(&self) -> &Vec<Job<Zombie>> {
        &self.zombie
    }

    /// Add to the scheduler
    ///
    /// The method takes in arguments a job. This job will be added in the processus HashMap.
    ///
    /// # Example :
    /// ```
    ///# use filasse::scheduler::*;
    ///# use filasse::job::*;
    /// let mut sched = Scheduler::default();
    /// let mut job = Job::default();
    /// sched.add_to_scheduler(&mut job);
    ///```
    pub fn add_to_scheduler(&mut self, job: &mut Job<New>) {
        let joba: Job<Ready> = Job::from(*job);
        self.queue.push(joba);
        self.pid_count += 1;
    }

    /// Process
    ///
    /// TBD
    ///
    /// # Example :
    /// ```
    ///# use filasse::scheduler::*;
    ///# use filasse::job::*;
    /// let mut sched = Scheduler::new(1);
    ///    let mut job = Job::new(1, 2, 2, 2);
    /// sched.add_to_scheduler(&mut job);
    /// sched.process();
    ///```
    pub fn process(&mut self) {
        let job = self.queue.get_mut(0).unwrap();
        let mut job: Job<Running> = Job::from(*job);
        if job.state.duration > 0 {
            if (job.state.duration - self.q) > 0 {
                job.state.duration -= self.q;
                let job = Job::from(job);
                self.queue.push(job);
                self.queue.remove(0);
            } else {
                job.state.duration = 0;
                let job = Job::from(job);
                self.zombie.push(job);
                self.queue.remove(0);
            }
        }
    }

    /// Algorithm Round robin
    ///
    /// TBD
    ///
    /// # Example :
    /// ```
    ///# use filasse::scheduler::*;
    ///# use filasse::job::*;
    /// let mut sched = Scheduler::new(1);
    /// let mut job = Job::new(1, 2, 2, 2);
    /// sched.add_to_scheduler(&mut job);
    /// sched.process();
    /// sched.round_robin();
    ///```
    pub fn round_robin(&mut self) {
        loop {
            if !self.queue.is_empty() {
                match self.queue.get(0) {
                    Some(_) => {
                        println!("SUR LE PROCESSUS : {:?}", self.queue[0]);
                        self.process(); // pendant Q temps
                    }
                    None => break,
                }
                println!("Queue : {:?}\nZombie : {:?}\n", self.queue(), self.zombie);
            } else {
                println!("No Processus");
                break;
            }
        }
    }
}
