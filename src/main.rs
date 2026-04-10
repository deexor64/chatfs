pub mod operations;
pub mod path_validation;
pub mod message_handler;

use std::env;
use dotenv::dotenv;
use tungstenite::{Message, connect};

use message_handler::message_handler::{handle_ping, handle_connect_ack, handle_query_codebase};
use message_handler::message_types::{ConnectAck, QueryCodebase};

// TODO: Make socket connections async
fn main() {
    // Load envars
    dotenv().ok();

    // Connect to socket
    let server_url = env::var("SERVER_URL").unwrap_or_else(|_| "ws://127.0.0.1:8000/client/".to_string());
    let (mut socket, _response) = connect(server_url).expect("Can't connect");

    loop {
        let message = socket.read().expect("Error reading message");

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
