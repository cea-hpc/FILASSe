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

extern "C" {
    fn fct_export();
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
        let mut ctx: ucontext_t = Self::create_ctx();
        unsafe {
            getcontext(&mut ctx as *mut ucontext_t);
        }
        ctx
    }

    pub fn set() -> ucontext_t {
        let mut ctx: ucontext_t = Self::create_ctx();
        unsafe {
            getcontext(&mut ctx as *mut ucontext_t);
        }
        unsafe {
            let _fct = Box::new(fct_export);
            makecontext(
                &mut ctx as *mut ucontext_t,
                mem::transmute::<unsafe extern "C" fn(), extern "C" fn()>(fct_export),
                0,
            );
        }
        ctx
    }

    /// Swap list
    ///
    ///```rust, ignore
    ///# use filasse::threads::*;
    ///# use nix::libc::ucontext_t;
    ///# use std::collections::VecDeque;
    ///let mut a = Thread{id: 0, current: Thread::create_ctx(), ready: VecDeque::new(), idle: VecDeque::from([Thread::create_ctx()]) };
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
    ///let mut current = Thread::get();
    ///let mut a =Thread{id: 0, current: current, ready: VecDeque::from([Thread::create_ctx()]), idle: VecDeque::from([Thread::create_ctx()]) };
    ///vp.0.push(a);
    ///
    ///vp.0.clone().swap_ctx(&mut vp);
    ///
    ///assert!(vp.0.current == Thread::create_ctx());
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

    /// Take work form another Virtual processor
    ///
    /// ```rust,ignore
    ///# use filasse::threads::*;
    ///# use std::collections::VecDeque;
    ///# use nix::libc::ucontext_t;
    ///# use std::sync::Arc;
    ///# use std::sync::Mutex;
    ///# let mut ctx: ucontext_t = Thread::create_ctx();
    ///# let mut tt = Thread {    id: 1,    current: ctx,    ready: VecDeque::new(),    idle: VecDeque::new()};
    ///# let mut tt2 = Thread {    id: 1,    current: ctx,    ready: VecDeque::new(),    idle: VecDeque::new()};
    ///# let _vp = vec![tt,tt2];
    ///# let vp = Arc::new(Mutex::new(_vp));
    ///vp.process[0].work_take(vp);
    // pub fn work_take(&mut self) {
    //     let mut _vp = &VPROCESSORS.0;
    //     let mut _v = _vp.lock().unwrap();
    //     _v.iter_mut().for_each(|x| {
    //         if x.id != self.id {
    //             if let Some(ctx) = x.idle.pop_front() {
    //                 x.idle.push_front(ctx);
    //             }
    //         }
    //     });
    //     drop(_v);
    // }

    pub fn ctx_yield(&mut self) {
        let mut _current: ucontext_t;
        let mut _next: Option<ucontext_t>;

        _current = self.current;
        _next = self.ready.pop_front();

        if let Some(_ctx) = _next {
            unimplemented!()
        } else {
            self.swap_ctx();
        }
        // thread_swap(_, _);
    }
}
