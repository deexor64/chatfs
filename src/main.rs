use std::process::exit;
use dotenv::dotenv;

use chatfs::cli::interface::cli_handler;
use chatfs::connection::socket::socket_loop;
use chatfs::utils::logger;

fn main() -> Result<(), ()> {
    // Load environment variables
    dotenv().ok();

    // Start CLI handler
    if let Err(e) = cli_handler() {
        logger::log_error(e);
        logger::log_info("Exiting...".to_string());
        exit(0);
    }

    // Start socket loop
    if let Err(e) = socket_loop() {
        logger::log_error(e);
    }

    logger::log_info("Exiting...".to_string());
    Ok(())
}
