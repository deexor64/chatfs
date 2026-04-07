pub mod ignorefile;
pub mod operations;
pub mod messagetypes;
pub mod replytypes;

use std::collections::HashMap;

use serde_json::Value;
use tungstenite::{Message, connect};
use operations::list::list;

use messagetypes::{ConnectAck, GetContext};
use replytypes::{ReplyType, CodeContext};

fn main() {
    let (mut socket, _response) = connect("ws://127.0.0.1:8000/client/").expect("Can't connect");
    // let (mut socket, _response) = connect("wss://querysync-server.onrender.com/client/").expect("Can't connect");
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
            // Initial connection
            if let Ok(parsed) =  serde_json::from_str::<ConnectAck>(&msg) {
                println!("status: {}", parsed.status);
                println!("message_type: {:?}", parsed.message_type);
                println!("server_url: {}", parsed.server_url);

            }
            // Code context
            else if let Ok(parsed) =  serde_json::from_str::<GetContext>(&msg) {
                println!("status: {}", parsed.status);
                println!("message_type: {:?}", parsed.message_type);
                println!("command: {:?}", parsed.command);
                println!("queries: {:?}", parsed.queries);

                match parsed.command {
                    _ => {
                        let queries: HashMap<String, Value> = parsed.queries;
                        
                        let recursive = queries.get("recursive")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false);
                        
                        let itemtype = queries.get("itemtype")
                            .and_then(|v| v.as_str())
                            .unwrap_or("all");
                        
                        let path = queries.get("path")
                            .and_then(|v| v.as_str())
                            .unwrap_or("");
                        
                        let list_dir = list(path, recursive, Some(itemtype), None);
                        
                        let response = CodeContext {
                            status: true,
                            reply_type: ReplyType::CodeContext,
                            context: list_dir.as_str()
                        };

                        // Convert to string
                        let response_text = serde_json::to_string(&response).unwrap();

                        // Send over the WebSocket
                        socket.send(Message::Text(response_text.into())).expect("Failed to send");
                    }
                }

            }
            // Invalid
            else {
                println!("Invalid JSON")
            }
        }
    }
}
