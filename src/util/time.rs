use std::time::Duration;

pub fn duration_millis(duration: Duration) -> u64 {
    (duration.subsec_nanos() / 1_000_000) as u64 +
        duration.as_secs() * 1000
}
