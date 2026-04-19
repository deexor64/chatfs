use serde_json::Value;
use tungstenite::{Bytes, Message, WebSocket, stream::MaybeTlsStream};
use std::path::PathBuf;
use std::{collections::HashMap, net::TcpStream};

use crate::logger;
use crate::message_handler::message_types::{Command, ConnectAck, QueryCodebase};
use crate::message_handler::reply_types::{CodeContext, MessageError, ReplyType};
use crate::operations;

fn find_ignore_file() -> Option<PathBuf> {
    let candidates = [".qsignore", ".fsignore", ".gitignore"];
    for file in candidates {
        let path = PathBuf::from(file);
        if path.exists() {
            return Some(path);
        }
    }
    None
}

// Regular ping message
pub fn handle_ping(message: Bytes, socket: &mut WebSocket<MaybeTlsStream<TcpStream>>){
    println!("Ping recieved..Sending heartbeat..  ˗ˋˏ ♡ ˎˊ˗");

    if let Err(err) = socket.send(Message::Pong(message)) {
        logger::log_warn(format!("Failed to send pong: {}", err));
    }

    if let Err(err) = socket.send(Message::Text("pong".into())) {
        // Do not remove this. The client object at server expects this text version of pong to expire the client object
        logger::log_warn(format!("Failed to send pong: {}", err));
    }
}

// Invalid message handler
pub fn handle_invalid_message(socket: &mut WebSocket<MaybeTlsStream<TcpStream>>){
    let response = MessageError {
        status: false,
        reply_type: ReplyType::MessageError,
        error: "Invalid json recieved".to_string()
    };

    let response_text = serde_json::to_string(&response).unwrap();

    socket.send(Message::Text(response_text.into())).unwrap();
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

    let ignore_file = find_ignore_file();

    match message.command {
        Command::List => context = operations::list::list(&queries, ignore_file.as_ref()),
        Command::Content => context = operations::content::content(&queries, ignore_file.as_ref()),
        Command::Create => context = operations::create::create(&queries, ignore_file.as_ref()),
        Command::Copy => context = operations::copy::copy(&queries, ignore_file.as_ref()),
        Command::Move => context = operations::mv::mv(&queries, ignore_file.as_ref()),
        Command::Delete => context = operations::delete::delete(&queries, ignore_file.as_ref()),
        Command::Write => context = operations::write::write(&queries, ignore_file.as_ref()),
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
