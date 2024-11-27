/// Convert bytes to a human-readable format
pub fn pretty_bytes(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} B", size)
    }
}

/// Convert seconds to a human-readable format
pub fn pretty_seconds(seconds: u64) -> String {
    const MINUTE: u64 = 60;
    const HOUR: u64 = MINUTE * 60;
    const DAY: u64 = HOUR * 24;

    if seconds >= DAY {
        format!("{:.2} D", seconds as f64 / DAY as f64)
    } else if seconds >= HOUR {
        format!("{:.2} H", seconds as f64 / HOUR as f64)
    } else if seconds >= MINUTE {
        format!("{:.2} M", seconds as f64 / MINUTE as f64)
    } else {
        format!("{} S", seconds)
    }
}
