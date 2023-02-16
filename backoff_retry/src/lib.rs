#[cfg(test)]
mod tests {
    use backoff::backoff::Backoff;
    use backoff::ExponentialBackoff;
    #[test]
    fn test_default() {
        let mut exp = ExponentialBackoff::default();

        for _ in 0..20 {
            dbg!(exp.current_interval);
            exp.next_backoff();
        }

        // [backoff_retry/src/lib.rs:11] exp.current_interval = 500ms
        // [backoff_retry/src/lib.rs:11] exp.current_interval = 750ms
        // [backoff_retry/src/lib.rs:11] exp.current_interval = 1.125s
        // [backoff_retry/src/lib.rs:11] exp.current_interval = 1.6875s
        // [backoff_retry/src/lib.rs:11] exp.current_interval = 2.53125s
        // [backoff_retry/src/lib.rs:11] exp.current_interval = 3.796875s
        // [backoff_retry/src/lib.rs:11] exp.current_interval = 5.6953125s
        // [backoff_retry/src/lib.rs:11] exp.current_interval = 8.54296875s
        // [backoff_retry/src/lib.rs:11] exp.current_interval = 12.814453125s
        // [backoff_retry/src/lib.rs:11] exp.current_interval = 19.221679687s
        // [backoff_retry/src/lib.rs:11] exp.current_interval = 28.83251953s
        // [backoff_retry/src/lib.rs:11] exp.current_interval = 43.248779295s
        // [backoff_retry/src/lib.rs:11] exp.current_interval = 60s
        // [backoff_retry/src/lib.rs:11] exp.current_interval = 60s
    }
}
