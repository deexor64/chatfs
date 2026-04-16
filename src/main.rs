pub mod tool_config;
pub mod operations;
pub mod path_validation;
pub mod message_handler;

use std::env;

use tool_config::config::{self, ConfigKey};
use message_handler::message_handler::{handle_ping, handle_connect_ack, handle_query_codebase};
use message_handler::message_types::{ConnectAck, QueryCodebase};

use tungstenite::{Message, connect};
use dotenv::dotenv;


fn main() {
    // Load environment variables
    dotenv().ok();

    // Load config
    config::load_config().map_err(|e| format!("ERROR: {}", e)).unwrap();

    // Temp
    let _server_url = env::var("SERVER_URL").map_err(|_| "".to_string()).unwrap_or("ws://127.0.0.1:8000/client/".to_string());
    config::set_config(ConfigKey::Gateway, &_server_url)
        .map_err(|e| format!("ERROR: {}", e))
        .unwrap();

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
