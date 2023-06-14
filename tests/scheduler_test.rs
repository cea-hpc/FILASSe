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
    fn add_to_queue() {
        let mut sched = Scheduler::default();
        sched.add_to_scheduler(&mut Job::default());
        sched.add_to_scheduler(&mut Job::default());
        sched.add_to_queue();
        assert!(sched.queue().clone() == vec![1, 2] || sched.queue().clone() == vec![2, 1])
    }

    #[test]
    fn add_to_scheduler() {
        let mut sched = Scheduler::default();
        sched.add_to_scheduler(&mut Job::default());
        assert!(sched.pid_count() == 2);
        assert!(sched.processus().get(&1).unwrap().state() == State::Ready);
    }

    #[test]
    fn is_scheduled() {
        let mut sched = Scheduler::default();
        sched.add_to_scheduler(&mut Job::default());
        assert!(sched.queue().is_empty());
        sched.add_to_queue();
        assert!(sched.queue().len() == 1);
    }

    #[test]
    fn process() {
        let mut sched = Scheduler::new(1);
        sched.add_to_scheduler(&mut Job::new(1, 2));
        sched.add_to_queue();
        sched.process(1);
        assert!(sched.processus_mut().get_mut(&1).unwrap().state() == State::Ready);
        sched.process(1);
        assert!(sched.processus_mut().get_mut(&1).unwrap().state() == State::Zombie);
    }
}
