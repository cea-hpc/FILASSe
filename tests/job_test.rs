#[cfg(test)]
mod tests {

    use filasse::job::*;

    #[test]
    fn default() {
        let foo = Job::default();
        assert!(foo.priority() == 0 && foo.duration() == 0);
    }

    #[test]
    fn new() {
        let foo = Job::new(1, 2);
        assert!(foo.priority() == 1 && foo.duration() == 2);
    }

    #[test]
    fn priority() {
        let mut foo = Job::default();
        foo.set_priority(3);
        assert!(foo.priority() == 3);
    }

    #[test]
    fn duration() {
        let mut foo = Job::default();
        foo.set_duration(3);
        assert!(foo.duration() == 3);
    }

    #[test]
    fn state() {
        let mut foo = Job::default();
        foo.set_state(State::Blocked);
        assert!(foo.state() == State::Blocked);
    }

    #[test]
    fn is_running() {
        let mut a = Job::default();
        a.ready();
        a.run();
        assert_eq!(State::Running, a.state());
    }

    #[test]
    fn is_unlocked() {
        let mut a = Job::default();
        a.set_state(State::Blocked);
        a.unlock();
        assert_eq!(State::Ready, a.state());
    }

    #[test]
    fn is_locked() {
        let mut a = Job::default();
        a.set_state(State::Running);
        a.lock();
        assert_eq!(State::Blocked, a.state());
    }

    #[test]
    fn is_zombie_run() {
        let mut a = Job::default();
        a.set_state(State::Running);
        a.zombie();
        assert_eq!(State::Zombie, a.state());
    }

    #[test]
    fn is_zombie_blocked() {
        let mut a = Job::default();
        a.set_state(State::Blocked);
        a.zombie();
        assert_eq!(State::Zombie, a.state());
    }

    #[test]
    fn is_finish() {
        let mut a = Job::default();
        a.set_state(State::Zombie);
        a.finish();
        assert_eq!(State::Terminated, a.state());
    }

    #[test]
    fn is_ready_default() {
        let mut a = Job::default();
        a.ready();
        assert_eq!(State::Ready, a.state());
    }

    #[test]
    fn is_ready_run() {
        let mut a = Job::default();
        a.set_state(State::Running);
        a.ready();
        assert_eq!(State::Ready, a.state());
    }
}
