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
