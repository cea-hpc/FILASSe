use nix::libc;
use std::mem::{self, transmute};

// TODO change stack size to allow default system stack size
const STACK_SIZE: usize = 40960;

/// Enum Task is the reprensentation of all states available to the scheduler
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Task {
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

unsafe impl Sync for Task {}

/// Struct Job reprensent the entity containing the context.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Job {
    pub parent: Option<libc::ucontext_t>,
    pub context: libc::ucontext_t,
}

impl Job {
    /// Use an existing job and define its function
    pub fn set_job_context(self, func: impl Fn()) -> Self {
        Job {
            context: self.set_context(func),
            parent: Some(self.context),
        }
    }

    /// Use to create a job without function. Alloc memory and create context.
    pub fn create_context() -> Self {
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

    pub fn swap_context(&mut self, dest: &mut libc::ucontext_t) {
        unsafe {
            libc::swapcontext(&mut self.context, dest);
        }
    }

    pub fn set_context(&self, func: impl Fn()) -> libc::ucontext_t {
        fn mkctx(func: *const u8) {
            let ptr: *const *const dyn Fn() = func as *const _;
            let fatptr: *const dyn Fn() = unsafe { *ptr };
            unsafe { (*fatptr)() }
            crate::filasse::SCHEDULER.yield_task();
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
