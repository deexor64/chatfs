use serde_json::Value;
use tungstenite::{Message, WebSocket, stream::MaybeTlsStream};
use std::net::TcpStream;

use super::types::{ConnectSyn, LlmCommand, InvalidLlmCommand, ConnectAck, Pong, LlmResult, ReplyType};
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

    println!(" ➤  Connected → {}\n\n", message.gateway_url);

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

    // println!("\n ـﮩﮩ٨ـ  Ping recieved, Heartbeat sent");

    Ok(())
}


pub fn handle_llm_command(message: LlmCommand, socket: &mut WebSocket<MaybeTlsStream<TcpStream>>) -> Result<(), String> {
    // Execute command and form reply
    let reply = match execute_command(&message.command, &message.params) {
        Ok(result) => LlmResult {
            id: message.id.clone(),
            status: true,
            reply_type: ReplyType::LlmResult,
            result: result
        },
        Err(err) => LlmResult {
            id: message.id.clone(),
            status: false,
            reply_type: ReplyType::LlmResult,
            result: Value::String(err)
        }
    };

    // Reply with LlmResult
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

    // TODO: Make this take only command and params
    println!("\n ✳  Command executed → {}", &message);

    Ok(())
}

pub fn handle_invalid_llm_command(message: InvalidLlmCommand, socket: &mut WebSocket<MaybeTlsStream<TcpStream>>)
-> Result<(), String> {
    // Execute command and form reply
    let reply = LlmResult {
        id: message.id,
        status: false,
        reply_type: ReplyType::LlmResult,
        result: Value::String(format!("Invalid command '{}'", &message.command))
    };

    // Reply with LlmResult
    match serde_json::to_string(&reply) {
        Ok(reply) => {
            if let Err(_) = socket.send(Message::Text(reply.into())) {
                return Err("Failed to send error for invalid command".to_string());
            }
        },
        Err(_) => {
            return Err("Failed to send error for invalid command".to_string())
        }
    }

    println!("\n ✕  Invalid command rejected → {}", message.command);

    Ok(())
}
