use nix::libc;
use std::{
    collections::VecDeque,
    mem::{self, transmute},
    sync::Mutex,
};

// TODO change stack size to allow default system stack size
const STACK_SIZE: usize = 40960;
/// Dummy struct for Scheduler
/// Even not enough to properly run a real theaded program as a task queue should be necessary
#[derive(Debug)]
struct Scheduler {
    queue: Mutex<VecDeque<Task>>,
    algorithm: Fifo,
    current: Mutex<Option<Task>>,
    idle: Mutex<Option<libc::ucontext_t>>,
}

trait Algorithm {
    fn get_next_task(&self, queue: &Mutex<VecDeque<Task>>) -> Option<Task>;
    fn add_to_queue(&self, queue: &Mutex<VecDeque<Task>>, task: Task);
}

#[derive(Debug)]
struct Fifo {}
impl Algorithm for Fifo {
    fn add_to_queue(&self, queue: &Mutex<VecDeque<Task>>, task: Task) {
        queue.lock().unwrap().push_back(task);
    }
    fn get_next_task(&self, queue: &Mutex<VecDeque<Task>>) -> Option<Task> {
        let mut guard = queue.lock().unwrap();
        guard.pop_front()
    }
}

#[derive(Debug)]
pub struct SchedulerUser {
    scheduler: Scheduler,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Job {
    parent: Option<libc::ucontext_t>,
    context: libc::ucontext_t,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Task {
    /// Creation State
    New(Job),
    // In Queue State, ready to be scheduled
    Ready(Job),
    // In run
    Running(Job),
    // Blocked by I/O
    Blocked(Job),
    // Father dead
    Zombie(Job),
    // Before to be free
    Terminated(Job),
}

impl SchedulerUser {
    pub fn create_task<F: Fn() -> ()>(&self, func: F) {
        self.scheduler.create_task(func);
    }

    pub fn join(&self) {
        self.yield_task();
    }

    pub fn yield_task(&self) {
        self.scheduler.yield_task();
    }

    pub fn lock(&self) {
        self.scheduler.lock();
    }
    pub fn unlock(&self) {
        self.scheduler.unlock();
    }
}

impl Scheduler {
    /// Register a task to run
    fn create_task<F: Fn() -> ()>(&self, func: F) {
        eprintln!("Task created");
        // Should only register the task, not run it.
        // When add to queue
        let lock = *self.current.lock().unwrap();
        if let Some(Task::Ready(job)) = lock {
            self.algorithm
                .add_to_queue(&self.queue, Task::Ready(job.set_job_context(func)));
        } else {
            drop(lock);
            let job = Job::create_context();
            self.idle.lock().unwrap().get_or_insert(job.context);
            self.current.lock().unwrap().get_or_insert(Task::Ready(job));
            self.algorithm
                .add_to_queue(&self.queue, Task::Ready(job.set_job_context(func)));
        }
    }
    /// Run a given task. Not to be called directly by end-user.
    fn run_task(&self) {
        self.destroy_task();
    }

    /// Yield: go back to scheduler to run one of the ready tasks.
    fn yield_task(&self) {
        eprintln!("Calling yield");
        std::thread::sleep(std::time::Duration::from_secs(1));
        // check if current, yes = running task -> ready ELSE ready/new to running

        let current = *self.current.lock().unwrap();
        let next = self.algorithm.get_next_task(&self.queue);

        if let Some(Task::Ready(mut _next)) = next {
            if let Some(Task::Ready(mut _current)) = &current {
                if !_current.eq(&_next) {
                    self.algorithm.add_to_queue(&self.queue, current.unwrap());
                    _current.swap_context(&mut _next.context)
                }
            }
        } else {
            if let Some(Task::Ready(mut _current)) = &current {
                let mut idle = self.idle.lock().unwrap().unwrap();
                _current.swap_context(&mut idle);
            }
        }
    }
    /// Destroy at end of task. Not to be called directly.
    fn destroy_task(&self) {
        println!("Terminating task");
    }

    // Task::Blocked
    fn lock(&self) {}
    // Task::Ready
    fn unlock(&self) {}
}

impl Job {
    fn set_job_context(&self, func: impl Fn()) -> Self {
        Job {
            context: self.set_context(func),
            parent: Some(self.context),
        }
    }

    fn create_context() -> Self {
        let mut context;
        unsafe {
            let mut stack = Vec::<u8>::new();
            stack.reserve(STACK_SIZE);
            context = mem::zeroed();

            libc::getcontext(&mut context);
            context.uc_stack = libc::stack_t {
                ss_sp: stack.as_mut_ptr() as *mut libc::c_void,
                ss_flags: 0,
                ss_size: STACK_SIZE,
            };
        };
        Job {
            parent: None,
            context,
        }
    }

    fn swap_context(&mut self, dest: &mut libc::ucontext_t) {
        unsafe {
            libc::swapcontext(&mut self.context, dest);
        }
    }

    fn set_context(&self, func: impl Fn()) -> libc::ucontext_t {
        fn mkctx(func: *const u8) {
            let ptr: *const *const dyn Fn() = func as *const _;
            let fatptr: *const dyn Fn() = unsafe { *ptr };
            unsafe { (*fatptr)() }
            SCHEDULER.yield_task();
        }
        let mut stack = Vec::<u8>::new();
        stack.reserve(STACK_SIZE);
        unsafe {
            let mut context: libc::ucontext_t = mem::zeroed();

            libc::getcontext(&mut context);
            context.uc_stack = libc::stack_t {
                ss_sp: stack.as_mut_ptr() as *mut libc::c_void,
                ss_flags: 0,
                ss_size: STACK_SIZE,
            };
            let fatptr: *const dyn Fn() = Box::leak(Box::new(func)) as *const _;
            let ptr: *const *const dyn Fn() = Box::leak(Box::new(fatptr)) as *const _;
            // Self::mkctx(ptr as *const _);
            libc::makecontext(
                &mut context,
                transmute::<fn(*const u8), extern "C" fn()>(mkctx),
                1,
                ptr,
            );

            context
        }
    }
}

pub static SCHEDULER: SchedulerUser = SchedulerUser {
    scheduler: Scheduler {
        queue: Mutex::new(VecDeque::new()),
        algorithm: Fifo {},
        current: Mutex::new(None),
        idle: Mutex::new(None),
    },
};
unsafe impl Sync for SchedulerUser {}
unsafe impl Sync for Task {}

fn task1() -> () {
    eprintln!("Starting task 1 execution");
    // Make scheduler able to schedule another ready task
    // Yield can also be hidden into a lock
    SCHEDULER.yield_task();

    eprintln!("Resuming task 1");
    // Creating a child task
    SCHEDULER.create_task(task2);

    eprintln!("Resuming task 1");
    // Make scheduler able to schedule another ready task
    SCHEDULER.yield_task();

    eprintln!("Ending task 1");
}

fn task2() -> () {
    eprintln!("Starting task 2");
    SCHEDULER.yield_task();

    eprintln!("Resuming task 2");
    SCHEDULER.create_task(|| {
        eprintln!("Executing a sub task");
    });
    eprintln!("Ending task 2");
}

fn task3() -> () {
    eprintln!("Executing task 3");
}

fn main() {
    SCHEDULER.create_task(task1);
    // SCHEDULER.create_task(task3);
    SCHEDULER.join();
    eprintln!("end of program");
}
