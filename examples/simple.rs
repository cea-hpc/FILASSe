use nix::libc;
use std::{
    collections::VecDeque,
    mem::{self, transmute, transmute_copy},
    sync::Mutex,
};

/// Dummy struct for Scheduler
/// Even not enough to properly run a real theaded program as a task queue should be necessary
struct Scheduler {
    queue: Mutex<VecDeque<Task>>,
    algorithm: Fifo,
    current: Option<Task>,
}

trait Algorithm {
    fn get_next_task(&self, queue: &Mutex<VecDeque<Task>>) -> Option<Task>;
    fn add_to_queue(&self, queue: &Mutex<VecDeque<Task>>, task: Task);
}

struct Fifo {}
impl Algorithm for Fifo {
    fn add_to_queue(&self, queue: &Mutex<VecDeque<Task>>, task: Task) {
        if let Task::New(job) = task {
            queue.lock().unwrap().push_back(Task::Ready(job));
        }
    }
    fn get_next_task(&self, queue: &Mutex<VecDeque<Task>>) -> Option<Task> {
        let mut guard = queue.lock().unwrap();
        guard.pop_front()
    }
}

pub struct SchedulerUser {
    scheduler: Scheduler,
}
struct Job {
    // func: Box<dyn Fn() -> ()>,
    context: libc::ucontext_t,
}

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
    pub fn create_task<F: Fn() -> () + 'static>(&self, func: F) {
        unsafe {
            self.scheduler.create_task(func);
        }
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
        unsafe {
            self.algorithm.add_to_queue(
                &self.queue,
                Task::New(Job {
                    // func: Box::new(my_func),
                    context: self.set_context(func),
                }),
            );
        }
        self.yield_task();
    }

    /// Run a given task. Not to be called directly by end-user.
    fn run_task(&self, job: Job) {
        // self.swap_context();
        self.destroy_task();
    }

    /// Yield: go back to scheduler to run one of the ready tasks.
    fn yield_task(&self) {
        eprintln!("Calling yield");
        // check if current, yes = running task -> ready ELSE ready/new to running
        // if let Some(task) = &self.current {
        //     self.algorithm.add_to_queue(&self.queue, *task);
        // }
        if let Some(_retrun) = self.algorithm.get_next_task(&self.queue) {
            if let Task::Ready(job) = _retrun {
                self.run_task(job);
                unsafe {
                    // self.swap_context(&mut current, &mut job.context);
                }
            } else {
                eprintln!("Task is not ready");
            }
        } else {
            eprintln!("Nothing to do, returning to previous execution");
        }
    }
    /// Destroy at end of task. Not to be called directly.
    fn destroy_task(&self) {
        println!("Terminating task");
    }

    unsafe fn leaker(func: impl Fn()) {
        fn mkctx(func: *const u8) {
            let ptr: *const *const dyn Fn() = func as *const _;
            let fatptr: *const dyn Fn() = unsafe { *ptr };
            dbg!("DEBUG");
            unsafe { (*fatptr)() }
        }
        const STACK_SIZE: usize = 4096;
        libc::malloc(mem::size_of::<usize>());
        let mut stack = Vec::new();
        stack.reserve(STACK_SIZE);
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
    }

    unsafe fn set_context(&self, func: impl Fn()) -> libc::ucontext_t {
        let mut context: libc::ucontext_t = std::mem::uninitialized();
        libc::getcontext(&mut context);
        // libc::getcontext(&mut context);
        // libc::makecontext(&mut context, transmute::<_, extern "C" fn()>(func), 0);
        Self::leaker(func);
        dbg!("Debug");
        context
    }

    unsafe fn swap_context(&self, src: &mut libc::ucontext_t, dest: &mut libc::ucontext_t) {
        libc::swapcontext(src, dest);
    }

    // Task::Blocked
    fn lock(&self) {}
    // Task::Ready
    fn unlock(&self) {}
}

pub static SCHEDULER: SchedulerUser = SchedulerUser {
    scheduler: Scheduler {
        queue: Mutex::new(VecDeque::new()),
        algorithm: Fifo {},
        current: None,
    },
};

unsafe impl Sync for Task {}
unsafe impl Send for Task {}

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

fn main() {
    SCHEDULER.create_task(task1);
}
