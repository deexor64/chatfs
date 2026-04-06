pub mod operations;
use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;
use tungstenite::{Message, connect};

fn main() {

    #[derive(Debug, Deserialize)]
    struct MessageType {
        status: bool,
        message_type: String,
        #[serde(flatten)]
        extra: HashMap<String, Value>,
    }

    let (mut socket, _response) = connect("ws://127.0.0.1:8000/client/").expect("Can't connect");
    println!("Websocket connected!");

    loop {
        let message = socket.read().expect("Error reading message");

        // Handle ping
        if let Message::Ping(msg) = message {
            println!("Received ping: {:?}", msg);
            socket.send(Message::Pong(msg)).expect("Could not send pong");
            socket.send(Message::Text("pong".into())).expect("");

        }
        // Handle other messages
        else if let Message::Text(msg) = message {
            if let Ok(parsed) =  serde_json::from_str::<MessageType>(&msg) {
                println!("status: {}", parsed.status);
                println!("message_type: {}", parsed.message_type);
                for (key, value) in parsed.extra {
                    println!("{}: {}", key, value);
                }
            } else {
                println!("Invalid JSON")
            }
        }
    }
}
