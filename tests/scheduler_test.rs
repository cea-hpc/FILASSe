#[cfg(test)]
mod tests {

    use filasse::job::*;
    use filasse::scheduler::*;

    #[test]
    fn new() {
        let sched = Scheduler::new(20);
        assert!(sched.quantum() == 20);
    }

    #[test]
    fn default() {
        let sched = Scheduler::default();
        assert!(sched.quantum() == 100);
    }

    #[test]
    fn set_quantum() {
        let mut sched = Scheduler::default();
        sched.set_quantum(1);
        assert!(sched.quantum() == 1)
    }

    #[test]
    fn get_pid() {
        let mut sched = Scheduler::default();
        sched.add_to_scheduler(&mut Job::default());
        sched.add_to_scheduler(&mut Job::default());
        assert!(sched.pid_count() == 3)
    }

    #[test]
    fn add_to_scheduler() {
        let mut sched = Scheduler::default();
        sched.add_to_scheduler(&mut Job::default());
        assert!(sched.pid_count() == 2);
        assert!(
            sched.queue().get(0).unwrap().state
                == Ready {
                    duration: 0,
                    priority: 0
                }
        );
    }

    #[test]
    fn is_scheduled() {
        let mut sched = Scheduler::default();
        sched.add_to_scheduler(&mut Job::default());
        assert!(sched.queue().len() == 1);
    }

    #[test]
    fn process() {
        let mut sched = Scheduler::new(1);
        sched.add_to_scheduler(&mut Job::new(1, 2, 1, 0));
        assert!(
            sched.queue().get(0).unwrap().state
                == Ready {
                    duration: 1,
                    priority: 0
                }
        );
        sched.process();
        assert!(sched.zombie().get(0).unwrap().state == Zombie {});
    }
}
