use tungstenite::{Message, WebSocket, stream::MaybeTlsStream};
use std::net::TcpStream;

use super::types::{ConnectSyn, LlmCommand, ConnectAck, Pong, LlmResult, ReplyType};
use crate::core::executor::execute_command;


pub fn handle_connect_syn(message: ConnectSyn, socket: &mut WebSocket<MaybeTlsStream<TcpStream>>) -> Result<(), String> {
    // Unsuccessful connection
    if message.status == false {
        return Err("Couldn't connect to gateway. Try again...".to_string());
    }

    // Successful connection
    println!(r#"
            __  __ __   ____  ______  _____  _____
           /  ]|  |  | /    ||      ||     |/ ___/
          /  / |  |  ||  o  ||      ||   __(   \_
         /  /  |  _  ||     ||_|  |_||  |_  \__  |
        /   \_ |  |  ||  _  |  |  |  |   _] /  \ |
        \     ||  |  ||  |  |  |  |  |  |   \    |
         \____||__|__||__|__|  |__|  |__|    \___|

    "#);

    // Reply with ConnectAck
    let reply = ConnectAck {
        status: true,
        reply_type: ReplyType::ConnectAck,
    };

    match serde_json::to_string(&reply) {
        Ok(reply) => {
            if let Err(_) = socket.send(Message::Text(reply.into())) {
                return Err("Failed to send connect ackknowledgement".to_string());
            }
        },
        Err(_) => {
            return Err("Failed to send connect ackknowledgement".to_string())
        }
    }

    println!(" ➤ Connected → {}\n\n", message.gateway_url);

    Ok(())
}


pub fn handle_ping(socket: &mut WebSocket<MaybeTlsStream<TcpStream>>) -> Result<(), String> {
    let reply = Pong {
        status: true,
        reply_type: ReplyType::Pong,
    };

    match serde_json::to_string(&reply) {
        Ok(reply) => {
            if let Err(_) = socket.send(Message::Text(reply.into())) {
                return Err("Failed to send pong".to_string());
            }
        },
        Err(_) => {
            return Err("Failed to send pong".to_string())
        }
    }

    println!("ـﮩﮩ٨ـ  Ping recieved. Hearbeat sent...");

    Ok(())
}


pub fn handle_llm_command(message: LlmCommand, socket: &mut WebSocket<MaybeTlsStream<TcpStream>>) -> Result<(), String> {
    // Execute command
    let result = execute_command(&message.command, &message.params)?;

    // Reply with LlmResult
    let reply = LlmResult {
        id: message.id.clone(),
        status: result.status,
        reply_type: ReplyType::LlmResult,
        result: result.result
    };

    match serde_json::to_string(&reply) {
        Ok(reply) => {
            if let Err(_) = socket.send(Message::Text(reply.into())) {
                return Err("Failed to send command execution result".to_string());
            }
        },
        Err(_) => {
            return Err("Failed to send command execution result".to_string())
        }
    }

    println!("\n✳ Command executed → {}", &message);

    Ok(())
}
