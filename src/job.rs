// Todo:
// Use Result to manage error

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum State {
    New,
    Ready,
    Running,
    Blocked,
    Zombie,
    Terminated,
}

/// Struct of the job
///
/// ```
/// # use filasse::job::*;
/// struct job {
///     state: State,
///     priority: u32,
///     duration: u64,
/// }
#[derive(Debug, Copy, Clone)]
pub struct Job {
    state: State,
    priority: u32,
    duration: u64,
}

impl Default for Job {
    fn default() -> Self {
        Job {
            state: State::New,
            priority: 0,
            duration: 0,
        }
    }
}

impl Job {
    /// Creates a new Job
    ///
    /// Takes two arguments which are the priority and the duration. The methode will give you back a job with a `State::New` and the values given. It alse exist the default method to create the job. With it, no argument are require and the value of priority and duration will be set to Zero.
    /// # Examples
    /// Example 1 :
    ///```
    /// # use filasse::job::*;
    /// let job = Job::new(1,2);
    ///```
    /// Example 2 :
    ///
    ///```
    /// # use filasse::job::*;
    /// let job = Job::default();
    ///```
    pub fn new(priority: u32, duration: u64) -> Self {
        Job {
            state: State::New,
            priority,
            duration,
        }
    }

    /// Get state value
    ///
    /// This method is used to get the value of the job's state
    ///
    /// # Example :
    /// ```
    /// # use filasse::job::*;
    /// let job = Job::default();
    /// let status = job.state();
    /// ```
    pub fn state(&self) -> State {
        self.state
    }

    /// Get priority value
    ///
    /// This method is used to get the value of the job's priority
    ///
    /// # Example :
    /// ```
    /// # use filasse::job::*;
    /// let job = Job::default();
    /// let status = job.priority();
    /// ```
    pub fn priority(&self) -> u32 {
        self.priority
    }

    /// Get duration value
    ///
    /// This method is used to get the value of the job's duration
    ///
    /// # Example :
    /// ```
    /// # use filasse::job::*;
    /// let job = Job::default();
    /// let status = job.duration();
    /// ```
    pub fn duration(&self) -> u64 {
        self.duration
    }

    /// Set state value
    ///
    /// This method is used to set the value of the job's state. It takes a state in arguments. The job must alse to be mutable.
    /// # Example :
    /// ```
    /// # use filasse::job::*;
    /// let mut job = Job::default();
    /// let status = job.set_state(State::Running);
    /// ```
    pub fn set_state(&mut self, value: State) {
        self.state = value;
    }

    /// Set priority value
    ///
    /// This method is used to set the value of the job's priority  It takes a priority which is a u32 type in arguments. The job must alse to be mutable.
    /// # Example :
    /// ```
    /// # use filasse::job::*;
    /// let mut job = Job::default();
    /// let status = job.set_priority(2);
    /// ```
    pub fn set_priority(&mut self, value: u32) {
        self.priority = value;
    }

    /// Set duration value
    ///
    /// This method is used to set the value of the job's duration  It takes a duration which is a u64 type in arguments. The job must alse to be mutable.
    /// # Example :
    /// ```
    /// # use filasse::job::*;
    /// let mut job = Job::default();
    /// let status = job.set_duration(2);
    /// ```
    pub fn set_duration(&mut self, value: u64) {
        self.duration = value;
    }

    /// Change job to ready
    ///
    /// This method allows the job to pass the Ready state. To become ready, the job must be in the New or Running state. The job must alse to be mutable.
    ///
    /// # Example :
    /// ```
    /// # use filasse::job::*;
    /// let mut job = Job::default(); // job = {status: State::New, priority: 0, duration: 0 }
    /// job.ready();
    /// ```
    pub fn ready(&mut self) {
        self.state = match self.state {
            State::New => State::Ready,
            State::Running => State::Ready,
            _ => self.state.clone(),
        }
    }

    /// Lock the job
    ///
    /// This method allows the job to pass the Blocked state. To become Blocked, the job must be in the Running state. The job must alse to be mutable.
    ///
    /// # Example :
    /// ```
    /// # use filasse::job::*;
    /// let mut job = Job::default(); // job = {status: State::New, priority: 0, duration: 0 }
    /// job.set_state(State::Running);
    /// job.lock();
    /// ```
    pub fn lock(&mut self) {
        self.state = match self.state {
            State::Running => State::Blocked,
            _ => self.state.clone(),
        }
    }

    /// Unlock the job
    ///
    /// This method allows the job to return in the Ready state. To be unlocked, the job must be in the Locked state. The job must alse to be mutable.
    ///
    /// # Example :
    /// ```
    /// # use filasse::job::*;
    /// let mut job = Job::default(); // job = {status: State::New, priority: 0, duration: 0 }
    /// job.set_state(State::Blocked);
    /// job.unlock();
    /// ```
    pub fn unlock(&mut self) {
        self.state = match self.state {
            State::Blocked => State::Ready,
            _ => self.state.clone(),
        }
    }

    /// Run the job
    ///
    /// This method allows the job to run and pass the Running state. To run, the job must be in the Ready state. The job must alse to be mutable.
    ///
    /// # Example :
    /// ```
    /// # use filasse::job::*;
    /// let mut job = Job::default(); // job = {status: State::New, priority: 0, duration: 0 }
    /// job.set_state(State::Ready);
    /// job.run();
    /// ```
    pub fn run(&mut self) {
        self.state = match self.state {
            State::Ready => State::Running,
            _ => self.state.clone(),
        }
    }

    /// Change job to Zombie
    ///
    /// This method allows the job to pass the Zombie state. To become Zombie, the job must be in the Running state. The job must alse to be mutable.
    ///
    /// # Example :
    /// ```
    /// # use filasse::job::*;
    /// let mut job = Job::default(); // job = {status: State::New, priority: 0, duration: 0 }
    /// job.set_state(State::Running);
    /// job.zombie();
    /// ```
    pub fn zombie(&mut self) {
        self.state = match self.state {
            State::Running => State::Zombie,
            State::Blocked => State::Zombie,
            _ => self.state.clone(),
        }
    }

    /// Unschedule the job
    ///
    /// This method allows the job to be unschedule after its job. To become Terminated, the job must be in the Zombie state. The job must alse to be mutable.
    ///
    /// # Example :
    /// ```
    /// # use filasse::job::*;
    /// let mut job = Job::default(); // job = {status: State::New, priority: 0, duration: 0 }
    /// job.set_state(State::Zombie);
    /// job.finish();
    /// ```
    pub fn finish(&mut self) {
        self.state = match self.state {
            State::Zombie => State::Terminated,
            _ => self.state.clone(),
        }
    }
}
