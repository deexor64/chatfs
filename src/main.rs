pub mod tool_config;
pub mod cli_handler;
pub mod message_handler;
pub mod operations;
pub mod path_validation;
pub mod logger;

use cli_handler::cli_handler::cli_handler;
use message_handler::message_handler::{handle_ping, handle_connect_ack, handle_query_codebase};
use message_handler::message_types::{ConnectAck, QueryCodebase};

use tungstenite::{Message, connect};
use dotenv::dotenv;

use crate::tool_config::config;
use crate::tool_config::config_types::ConfigKey;


fn main() {
    // Load environment variables
    dotenv().ok();

    // CLI handler
    if let Err(e) = cli_handler() {
        logger::log_error(e);
        return;
    }

    // Message handler

    // Connect to socket
    let gateway_url = config::get_config(ConfigKey::Gateway).map_err(|e| format!("ERROR: {}", e)).unwrap();
    let (mut socket, _response) = connect(gateway_url).map_err(|e| format!("ERROR: {}", e)).unwrap();

    loop {
        let message = socket.read().expect("ERROR: Failed to read incoming message");

        // Handle ping
        if let Message::Ping(msg) = message {
            handle_ping(msg, &mut socket);
        }
        // Handle other messages
        else if let Message::Text(msg) = message {
            if let Ok(parsed) =  serde_json::from_str::<ConnectAck>(&msg) {
                handle_connect_ack(parsed);
            }
            else if let Ok(parsed) =  serde_json::from_str::<QueryCodebase>(&msg) {
                handle_query_codebase(parsed, &mut socket);
            }
            // Invalid
            else {
                println!("Invalid JSON")
            }
        }
    }
}
