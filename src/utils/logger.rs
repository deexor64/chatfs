use std::sync::atomic::{AtomicBool, Ordering};

static LOG_ENABLED: AtomicBool = AtomicBool::new(true);
static DEBUG_ENABLED: AtomicBool = AtomicBool::new(false);

// Toggle logging ON/OFF
pub fn toggle_logging(enabled: bool) {
    LOG_ENABLED.store(enabled, Ordering::Relaxed);
}

// Toggle debug logging ON/OFF
pub fn toggle_debug(enabled: bool) {
    DEBUG_ENABLED.store(enabled, Ordering::Relaxed);
}

// Helper to check if logging is enabled
fn is_log_enabled() -> bool {
    LOG_ENABLED.load(Ordering::Relaxed)
}

// Helper to check if debug logging is enabled
fn is_debug_enabled() -> bool {
    DEBUG_ENABLED.load(Ordering::Relaxed)
}


// Logs are printed if logging is enabled

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
    // Debug messages are only printed if debug logging is enabled
    if !is_debug_enabled() || !is_log_enabled(){
        return;
    }
    println!("\x1b[34m[DEBUG]\x1b[0m {}", msg);
}

pub fn log_error(msg: String) {
    // Error are always printed regardless of log level
    println!("\x1b[31m[ERROR]\x1b[0m {}", msg);
}
