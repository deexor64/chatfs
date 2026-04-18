use std::sync::atomic::{AtomicBool, Ordering};

static LOG_ENABLED: AtomicBool = AtomicBool::new(false);

// Toggle logging ON
pub fn enable_logging() {
    LOG_ENABLED.store(true, Ordering::Relaxed);
}

// Toggle logging OFF
pub fn disable_logging() {
    LOG_ENABLED.store(false, Ordering::Relaxed);
}

// Helper to check if logging is enabled
fn is_log_enabled() -> bool {
    LOG_ENABLED.load(Ordering::Relaxed)
}

pub fn log_info(msg: String) {
    if !is_log_enabled() {
        return;
    }
    println!("\x1b[32m[INFO]\x1b[0m {}", msg);
}

pub fn log_warn(msg: String) {
    if !is_log_enabled() {
        return;
    }
    println!("\x1b[33m[WARN]\x1b[0m {}", msg);
}

pub fn log_debug(msg: String) {
    if !is_log_enabled() {
        return;
    }
    println!("\x1b[34m[DEBUG]\x1b[0m {}", msg);
}

pub fn log_error(msg: String) {
    // Error are always printed regardless of log level
    println!("\x1b[31m[ERROR]\x1b[0m {}", msg);
}
