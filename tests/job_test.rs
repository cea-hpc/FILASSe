#[cfg(test)]
mod tests {
    use filasse::job::Job;

    #[test]
    fn create() {
        let job = Job::create_context();
        assert!(job.parent == None);
    }
}
