#[cfg(test)]
mod tests {

    use filasse::job::*;

    #[test]
    fn from_new_to_ready() {
        let bar = Job {
            pid: 1,
            parent: 0,
            state: New {
                duration: 2,
                priority: 2,
            },
        };
        let bar: Job<Ready> = bar.into();
        assert!(
            bar.state
                == Ready {
                    duration: 2,
                    priority: 2
                }
        );
    }

    #[test]
    fn from_ready_to_running() {
        let bar = Job {
            pid: 1,
            parent: 0,
            state: Ready {
                duration: 2,
                priority: 2,
            },
        };
        let bar: Job<Running> = bar.into();
        assert!(
            bar.state
                == Running {
                    duration: 2,
                    priority: 2
                }
        );
    }

    #[test]
    fn from_running_to_blocked() {
        let bar = Job {
            pid: 1,
            parent: 0,
            state: Running {
                duration: 2,
                priority: 2,
            },
        };
        let bar: Job<Blocked> = bar.into();
        assert!(
            bar.state
                == Blocked {
                    duration: 2,
                    priority: 2
                }
        );
    }

    #[test]
    fn from_running_to_ready() {
        let bar = Job {
            pid: 1,
            parent: 0,
            state: Running {
                duration: 2,
                priority: 2,
            },
        };
        let bar: Job<Ready> = bar.into();
        assert!(
            bar.state
                == Ready {
                    duration: 2,
                    priority: 2
                }
        );
    }

    #[test]
    fn from_running_to_zombie() {
        let bar = Job {
            pid: 1,
            parent: 0,
            state: Running {
                duration: 2,
                priority: 2,
            },
        };
        let bar: Job<Zombie> = bar.into();
        assert!(bar.state == Zombie {});
    }

    #[test]
    fn from_blocked_to_ready() {
        let bar = Job {
            pid: 1,
            parent: 0,
            state: Blocked {
                duration: 2,
                priority: 2,
            },
        };
        let bar: Job<Ready> = bar.into();
        assert!(
            bar.state
                == Ready {
                    duration: 2,
                    priority: 2
                }
        );
    }

    #[test]
    fn from_zombie_to_terminated() {
        let bar = Job {
            pid: 1,
            parent: 0,
            state: Zombie {},
        };
        let bar: Job<Terminated> = bar.into();
        assert!(bar.state == Terminated {});
    }
}
