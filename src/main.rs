use filasse::job::*;
use filasse::scheduler::*;

fn main() {
    let mut sched = Scheduler::new(1);
    let mut job1 = Job::new(1, 2);
    let mut job2 = Job::new(1, 3);
    let mut job3 = Job::new(1, 4);

    sched.add_to_scheduler(&mut job1);
    sched.add_to_scheduler(&mut job2);
    sched.add_to_scheduler(&mut job3);

    sched.add_to_queue();

    sched.round_robin();
}
