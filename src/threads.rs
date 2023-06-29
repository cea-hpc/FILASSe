use crate::job::*;
use nix::libc::{getcontext, makecontext, swapcontext, ucontext_t};
use std::sync::Mutex;
use std::{collections::VecDeque, mem};

#[repr(transparent)]
pub struct VirtualProcessor(Vec<Thread>);

#[derive(Debug)]
pub struct Thread {
    pub id: u64,
    pub current: ucontext_t,
    pub ready: VecDeque<ucontext_t>,
    pub idle: Mutex<VecDeque<ucontext_t>>,
    // pub job: Job<dyn State>,
}
unsafe impl Send for VirtualProcessor {}
unsafe impl Sync for VirtualProcessor {}
static VPROCESSORS: VirtualProcessor = VirtualProcessor(Vec::new());

impl Thread {
    /// Create context zeroed
    ///
    /// The fonction is used to create a zeroed context
    ///```rust, ignore
    ///# use filasse::threads::*;
    ///# use nix::libc::ucontext_t;
    ///let ctx = CtxT::create_ctx();
    ///```
    pub fn create_ctx() -> ucontext_t {
        let ctx: ucontext_t;
        unsafe {
            ctx = std::mem::MaybeUninit::<ucontext_t>::zeroed().assume_init();
        };
        ctx
    }

    pub fn get() -> ucontext_t {
        let mut ctx: ucontext_t = Self::create_ctx();
        unsafe {
            getcontext(&mut ctx as *mut ucontext_t);
        }
        ctx
    }
    /// Swap list
    ///
    ///```rust, ignore
    ///# use filasse::threads::*;
    ///# use nix::libc::ucontext_t;
    ///# use std::collections::VecDeque;
    ///let mut a = CtxT{id: 0, current: CtxT::create_ctx(), ready: VecDeque::new(), idle: VecDeque::from([CtxT::create_ctx()]) };
    ///a.swap_list();
    ///assert!(a.idle.is_empty());
    ///assert!(!a.ready.is_empty());
    ///```
    pub fn swap_list(&mut self) {
        let mut _idle = self.idle.lock().unwrap();
        if !_idle.is_empty() {
            self.ready = _idle.clone();
            _idle.clear();
        } else {
            // self.work_take(self);
        }
    }

    /// Swap the context
    ///
    /// Allow to change the current context with another context
    ///```rust, no_run, ignore
    ///# use filasse::threads::*;
    ///# use nix::libc::ucontext_t;
    ///# use std::collections::VecDeque;
    ///let mut vp = VP(Vec::new());
    ///let mut current = CtxT::get();
    ///let mut a =CtxT{id: 0, current: current, ready: VecDeque::from([CtxT::create_ctx()]), idle: VecDeque::from([CtxT::create_ctx()]) };
    ///vp.0.push(a);
    ///
    ///vp.0.clone().swap_ctx(&mut vp);
    ///
    ///assert!(vp.0.current == CtxT::create_ctx());
    ///```
    pub fn swap_ctx(&mut self) {
        if let Some(mut next) = self.ready.pop_front() {
            unsafe {
                swapcontext(
                    &mut self.current as *mut ucontext_t,
                    &mut next as *mut ucontext_t,
                );
            }
            self.current = next;
        } else {
            self.swap_list();
        }
    }
}
