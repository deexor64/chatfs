use dotenv::dotenv;

use chatfs::cli_handler::cli_handler::cli_handler;
use chatfs::logger::log_error;
use std::process::exit;

fn main() {
    // Load environment variables
    dotenv().ok();

    // Start CLI handler
    if let Err(e) = cli_handler() {
        log_error(e);
        exit(1);
    }
}
