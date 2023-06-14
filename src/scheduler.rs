use crate::job::*;
use std::collections::HashMap;

/// Struct of the scheduler
///
///```
///# use filasse::job::*;
///# use std::collections::HashMap;
/// pub struct Scheduler {
///     processus: HashMap<u64, Job>,
///     queue: Vec<u64>,
///     q: u64,
///     pid_count: u64,
///     available: bool,
/// }
///```

#[derive(Debug, Clone)]
pub struct Scheduler {
    processus: HashMap<u64, Job>,
    queue: Vec<u64>,
    q: u64,
    pid_count: u64,
    available: bool,
}

impl Default for Scheduler {
    fn default() -> Self {
        Scheduler {
            processus: HashMap::<u64, Job>::new(),
            queue: Vec::<u64>::new(),
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
            processus: HashMap::<u64, Job>::new(),
            queue: Vec::<u64>::new(),
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

    /// Getter processus
    ///
    /// The method allows you to get the processus HashMap
    ///
    /// # Example :
    /// ```
    ///# use filasse::scheduler::*;
    /// let sched = Scheduler::default();
    /// sched.processus();
    ///```
    pub fn processus(&self) -> &HashMap<u64, Job> {
        &self.processus
    }

    /// Getter mutable processus
    ///
    /// The method allows you to get the mutable processus HashMap
    ///
    /// # Example :
    /// ```
    ///# use filasse::scheduler::*;
    /// let mut sched = Scheduler::default();
    /// sched.processus_mut();
    ///```
    pub fn processus_mut(&mut self) -> &mut HashMap<u64, Job> {
        &mut self.processus
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
    pub fn queue(&self) -> &Vec<u64> {
        &self.queue
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
    pub fn add_to_scheduler(&mut self, job: &mut Job) {
        match job.state() {
            State::New => {
                job.ready();
                self.processus.insert(self.pid_count, *job);
                self.pid_count += 1;
            }
            _ => (),
        };
    }

    /// Add to the queue
    ///
    /// The method adds all the processus no already added to the queue if they are in the `State::Ready`
    ///
    /// # Example :
    /// ```
    ///# use filasse::scheduler::*;
    /// let mut sched = Scheduler::default();
    /// sched.add_to_queue();
    ///```
    pub fn add_to_queue(&mut self) {
        for job in &self.processus {
            if !self.queue.contains(job.0) {
                match job.1.state() {
                    State::Ready => self.queue.push(*job.0),
                    _ => (),
                }
            }
        }
    }

    /// Process
    ///
    /// TBD
    ///
    /// # Example :
    /// ```
    ///# use filasse::scheduler::*;
    ///# use filasse::job::*;
    /// let mut sched = Scheduler::default();
    /// let mut job = Job::default();
    /// sched.add_to_scheduler(&mut job);
    /// sched.add_to_queue();
    /// sched.process(1);
    ///```
    pub fn process(&mut self, job_pid: u64) {
        let job = self.processus.get_mut(&job_pid).unwrap();
        job.run();
        self.queue.remove(0);
        if job.duration() > 0 {
            if (job.duration() - self.q) > 0 {
                job.set_duration(job.duration() - self.q);
                self.queue.push(job_pid);
                job.ready();
            } else {
                job.set_duration(0);
                job.zombie();
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
    /// let mut sched = Scheduler::default();
    /// let mut job = Job::default();
    /// sched.add_to_scheduler(&mut job);
    /// sched.round_robin();
    ///```
    pub fn round_robin(&mut self) {
        loop {
            if !self.queue.is_empty() {
                match self.processus.get(&self.queue[0]) {
                    Some(job) => {
                        match job.state() {
                            State::Ready => {
                                println!("SUR LE PROCESSUS : {}", self.queue[0]);
                                self.process(self.queue[0]); // pendant Q temps
                            }
                            _ => (),
                        }
                    }
                    None => break,
                }
                println!(
                    "Queue : {:?}\nProcessus : {:?}\n",
                    self.queue(),
                    self.processus()
                );
            } else {
                println!("No Processus");
                break;
            }
        }
    }
}
