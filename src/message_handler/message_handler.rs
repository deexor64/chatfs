use serde_json::Value;
use tungstenite::{Bytes, Message, WebSocket, stream::MaybeTlsStream};
use std::path::PathBuf;
use std::{collections::HashMap, net::TcpStream};

use crate::message_handler::message_types::{Command, ConnectAck, QueryCodebase};
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
        println!("[UNSUCCESSFUL] → http://couldn't connect to gateway/");
        return;
    }
    println!(r#"
            __  __ __   ____  ______  _____  _____
           /  ]|  |  | /    ||      ||     |/ ___/
          /  / |  |  ||  o  ||      ||   __(   \_
         /  /  |  _  ||     ||_|  |_||  |_  \__  |
        /   \_ |  |  ||  _  |  |  |  |   _] /  \ |
        \     ||  |  ||  |  |  |  |  |  |   \    |
         \____||__|__||__|__|  |__|  |__|    \___|

    "#);

    println!("[CONNECTED] → {}\n\n", message.server_url);
}

// Request for codebase contexts
pub fn handle_query_codebase(message: QueryCodebase, socket: &mut WebSocket<MaybeTlsStream<TcpStream>>) {
    let id: String = message.id;
    let queries: HashMap<String, Value> = message.queries;
    let context: Value;

    // Ignore file
    // ISSUE: Doesn't work
    let ignorefile = if PathBuf::from(".qsignore").exists() {
            Some(".qsignore")
        } else {
            None
        };

    match message.command {
        Command::List => context = operations::list::list(&queries, ignorefile),
        Command::Content => context = operations::content::content(&queries, ignorefile),
        Command::Create => context = operations::create::create(&queries, ignorefile),
        Command::Copy => context = operations::copy::copy(&queries, ignorefile),
        Command::Move => context = operations::mv::mv(&queries, ignorefile),
        Command::Delete => context = operations::delete::delete(&queries, ignorefile),
        Command::Write => context = operations::write::write(&queries, ignorefile),
    }

    let response = CodeContext {
        id: id,
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
