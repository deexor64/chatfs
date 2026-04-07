use serde_json::Value;
use tungstenite::{Bytes, Message, WebSocket, stream::MaybeTlsStream};
use std::{collections::HashMap, net::TcpStream};

use crate::message_handler::message_types::{Command, ConnectAck, GetContext};
use crate::message_handler::reply_types::{ReplyType, CodeContext};
use crate::operations;

// Regular ping message
pub fn handle_ping(message: Bytes, socket: &mut WebSocket<MaybeTlsStream<TcpStream>>){
    socket.send(Message::Pong(message)).expect("Pong to failed");
    socket.send(Message::Text("pong".into())).expect("Pong to server failed");

    println!("Ping recieved..Sending heartbeat..  ˗ˋˏ ♡ ˎˊ˗");
}

// Initial connection acknowledgement
pub fn handle_connect_ack(message: ConnectAck) {
    if message.status == false {
        println!("[UNSUCCESSFUL] → https://couldn't connect to server/");
        return;
    }
    println!(r#"
        ___                    ___
       / _ \ _  _ ___ _ _ _  _/ __|_  _ _ _  __
      | (_) | || / -_) '_| || \__ \ || | ' \/ _|
       \__\_\\_,_\___|_|  \_, |___/\_, |_||_\__|
                          |__/     |__/
    "#);

    println!("[CONNECTED] → {}\n\n", message.server_url);
}

// Request for codebase contexts
pub fn handle_get_context(message: GetContext, socket: &mut WebSocket<MaybeTlsStream<TcpStream>>) {
    let queries: HashMap<String, Value> = message.queries;
    let context: Value;

    match message.command {
        Command::List => { context = operations::list::list(&queries, None) }
        _ => { context = serde_json::json!({
            "status": false,
            "reply_type": ReplyType::CodeContext,
            "context": "Not available"
        })}
    }

    let response = CodeContext {
        status: true,
        reply_type: ReplyType::CodeContext,
        context: context
    };

    // Convert to JSON string
    let response_text = serde_json::to_string(&response).unwrap();

    // Send over the WebSocket
    socket.send(Message::Text(response_text.into())).expect("Failed to send");
    println!("✳ Context sent: {:?} {:?}", message.command, queries);
}
