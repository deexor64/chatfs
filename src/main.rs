pub mod path_guards;
pub mod operations;
pub mod message_handler;

use tungstenite::{Message, connect};

use message_handler::message_handler::{handle_ping, handle_connect_ack, handle_query_codebase};
use message_handler::message_types::{ConnectAck, QueryCodebase};

fn main() {
    let (mut socket, _response) = connect("ws://127.0.0.1:8000/client/").expect("Can't connect");
    // let (mut socket, _response) = connect("wss://querysync-server.onrender.com/client/").expect("Can't connect");

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
