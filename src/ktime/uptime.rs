use super::ktime_seconds;

/// Return uptime and idle time
/// Don't return idle time for now
pub fn uptime() -> usize {
    let time = ktime_seconds();
    time as usize
}
