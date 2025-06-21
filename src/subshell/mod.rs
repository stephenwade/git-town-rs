use std::time::Duration;

pub mod backend_runner;

const CONCURRENT_GIT_RETRIES: i8 = 5;
const CONCURRENT_GIT_RETRY_DELAY: Duration = Duration::from_secs(1);
