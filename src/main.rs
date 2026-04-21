use dotenv::dotenv;

use chatfs::cli::interface::cli_handler;
use chatfs::transport::socket::socket_loop;

fn main() {
    // Load environment variables
    dotenv().ok();

    // Start CLI handler
    if let Err(e) = cli_handler() {
        panic!("{}", e);
    }

    // Start socket loop
    if let Err(e) = socket_loop() {
        panic!("{}", e);
    }
}
