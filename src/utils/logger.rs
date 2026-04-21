use std::sync::atomic::{AtomicBool, Ordering};

/*
 * This module provides a simple set of logging utilities for the tool
 * Logger assumes a single-threaded environment
 */

// Error logs are always printed regardless of log level
// Process may terminate after an error
pub fn log_error(msg: String) {
    println!("\x1b[31m[ERROR]\x1b[0m {}", msg);
}

// General logs are printed based on the log level
static LOG_ENABLED: AtomicBool = AtomicBool::new(true);

pub fn toggle_logging(enabled: bool) {
    LOG_ENABLED.store(enabled, Ordering::Relaxed);
}

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

// Debug logs are printed based on the debug log level
static DEBUG_ENABLED: AtomicBool = AtomicBool::new(false);

pub fn toggle_debug(enabled: bool) {
    DEBUG_ENABLED.store(enabled, Ordering::Relaxed);
}

fn is_debug_enabled() -> bool {
    DEBUG_ENABLED.load(Ordering::Relaxed)
}

pub fn log_debug(msg: String) {
    if !is_debug_enabled() || !is_log_enabled(){
        return;
    }
    println!("\x1b[34m[DEBUG]\x1b[0m {}", msg);
}
