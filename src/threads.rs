use crate::job::*;
use nix::libc::{getcontext, makecontext, swapcontext, ucontext_t};
use std::collections::VecDeque;
use std::sync::Mutex;

#[repr(transparent)]
pub struct VirtualProcessor(pub Vec<Thread>);

pub struct Thread {
    pub id: u64,
    pub current: Job,
    pub ready: VecDeque<Job>,
    pub idle: Mutex<VecDeque<Job>>,
}

unsafe impl Send for VirtualProcessor {}
unsafe impl Sync for VirtualProcessor {}
pub static mut VPROCESSORS: VirtualProcessor = VirtualProcessor(Vec::new());

fn algorithm_converter(function: fn()) -> extern "C" fn() {
    unsafe { std::mem::transmute::<fn(), extern "C" fn()>(function) }
}

/// Create context zeroed
///
/// The fonction is used to create a zeroed context
///```rust, ignore
///# use filasse::threads::*;
///# use nix::libc::ucontext_t;
///let ctx = Thread::create_ctx();
///```
pub fn create_ctx() -> ucontext_t {
    let ctx: ucontext_t;
    unsafe {
        ctx = std::mem::MaybeUninit::<ucontext_t>::zeroed().assume_init();
    };
    ctx
}

pub fn get() -> ucontext_t {
    let mut ctx: ucontext_t = create_ctx();
    unsafe {
        getcontext(&mut ctx as *mut ucontext_t);
    }
    ctx
}

// TODO
pub fn set_context(algorithm: extern "C" fn(), argc: i32) -> ucontext_t {
    let mut ctx: ucontext_t = create_ctx();
    unsafe {
        getcontext(&mut ctx as *mut ucontext_t);
    }
    unsafe {
        makecontext(&mut ctx as *mut ucontext_t, algorithm, argc);
    }
    ctx
}
impl Thread {
    /// Swap list
    ///
    ///```rust, ignore
    ///# use filasse::threads::*;
    ///# use nix::libc::ucontext_t;
    ///# use std::collections::VecDeque;
    ///# use std::sync::Mutex;
    /// let mut vp: VirtualProcessor = VirtualProcessor(Vec::new());
    ///let mut current = Thread::get();
    ///let mut a =Thread{id: 0, current: current, ready: VecDeque::new(), idle: Mutex::new(VecDeque::from([Thread::create_ctx()])) };
    ///vp.0.push(a);
    ///vp.0[0].swap_list();
    ///assert!(vp.0[0].current == Thread::create_ctx());
    ///```
    pub fn swap_list(&mut self) {
        let mut is_emp = true;
        if let Ok(_idle) = self.idle.lock() {
            if !_idle.is_empty() {
                is_emp = false;
                self.ready = _idle.clone();
            }
        }
        if is_emp {
            self.work_take();
        }
        self.ctx_yield();
    }

    /// Swap the context
    ///
    /// Allow to change the current context with another context
    ///```rust, no_run, ignore
    ///# use filasse::threads::*;
    ///# use std::sync::Mutex;
    ///# use nix::libc::ucontext_t;
    ///# use std::collections::VecDeque;
    /// let mut vp: VirtualProcessor = VirtualProcessor(Vec::new());
    ///let mut current = Thread::get();
    ///let mut a =Thread{id: 0, current: current, ready: VecDeque::from([Thread::create_ctx()]), idle: Mutex::new(VecDeque::from([Thread::create_ctx()])) };
    ///vp.0.push(a);
    ///let next = vp.0[0].ready.pop_front().unwrap();
    ///vp.0[0].swap_ctx(next);
    ///
    ///assert!(vp.0[0].current == Thread::create_ctx());
    ///```
    pub fn swap_ctx(&mut self, next: Job) {
        match (self.current, next) {
            (Job::Running(_, mut _current), Job::Ready(_, mut _next)) => unsafe {
                swapcontext(
                    &mut _current.context as *mut ucontext_t,
                    &mut _next.context as *mut ucontext_t,
                );
            },
            _ => panic!("Bad state of job"),
        }
    }

    /// Take work form another Virtual processor
    ///
    /// ```rust,ignore
    ///# use filasse::threads::*;
    ///# use std::collections::VecDeque;
    ///# use nix::libc::ucontext_t;
    ///# use std::sync::Mutex;
    ///# let mut ctx: ucontext_t = Thread::create_ctx();
    ///# let mut tt = Thread {    id: 1,    current: Thread::create_ctx(),    ready: VecDeque::new(),    idle: Mutex::new(VecDeque::new())};
    ///# let mut tt2 = Thread {    id: 2,    current: Thread::create_ctx(),    ready: VecDeque::new(),    idle: Mutex::new(VecDeque::from([ctx]))};
    ///unsafe {
    ///VPROCESSORS.0.push(tt);
    ///VPROCESSORS.0.push(tt2);
    ///VPROCESSORS.0[0].work_take();
    ///assert!(VPROCESSORS.0[0].idle.lock().unwrap().is_empty() == false);
    ///}
    pub fn work_take(&mut self) {
        unsafe {
            let mut _vp = &VPROCESSORS.0;
            _vp.iter().for_each(|x| {
                if self.id != x.id {
                    if let Some(_next) = x.idle.lock().unwrap().pop_front() {
                        let mut _current = self.idle.lock().unwrap();
                        _current.push_back(_next);
                        drop(_current);
                        self.ctx_yield();
                    }
                }
            });
        }
    }

    /// Take work form another Virtual processor
    /// The example cannot run dut to swap_ctx.
    ///
    /// ```rust,ignore
    ///# use filasse::threads::*;
    ///# use std::collections::VecDeque;
    ///# use nix::libc::ucontext_t;
    ///# use std::sync::Mutex;
    ///# let mut ctx: ucontext_t = Thread::create_ctx();
    ///# let mut tt = Thread {    id: 1,    current: Thread::create_ctx(),    ready: VecDeque::new(),    idle: Mutex::new(VecDeque::new())};
    ///# let mut tt2 = Thread {    id: 2,    current: Thread::create_ctx(),    ready: VecDeque::new(),    idle: Mutex::new(VecDeque::from([ctx]))};
    ///unsafe {
    ///VPROCESSORS.0.push(tt);
    ///VPROCESSORS.0.push(tt2);
    ///VPROCESSORS.0[0].ctx_yield();
    /// dbg!(&VPROCESSORS);
    ///assert!(VPROCESSORS.0[0].idle.lock().unwrap().is_empty() == false);
    ///}
    pub fn ctx_yield(&mut self) {
        let mut _current: Job;
        let mut _next: Option<Job>;

        _current = self.current;
        _next = self.ready.pop_front();

        if let Some(next) = _next {
            self.idle.lock().unwrap().push_back(_current);
            self.current = next;
            self.swap_ctx(next);
        } else {
            self.swap_list();
        }
    }
}
