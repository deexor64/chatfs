use std::time::Duration;
use tungstenite::{Message, connect};

use crate::logger;
use crate::message_handler::message_handlers::{handle_connect_ack, handle_invalid_message, handle_ping, handle_query_codebase};
use crate::message_handler::message_types::{ConnectAck, QueryCodebase};

const MAX_LISTEN_RETRIES: usize = 5;

pub fn socket_loop(gateway: String) -> Result<(), String> {
    // Gateway not set
    if gateway.is_empty() {
        logger::log_info("Gateway url is not set (use 'config-set gateway <URL>' or '--gateway <URL>' to set)".to_string());
        return Ok(());
    }

    // Connect to socket
    let (mut socket, _response) = connect(&gateway).map_err(|_| "Failed to connect gateway (check gateway url using 'config-get gateway')".to_string())?;
    logger::log_info(format!("Connected to gateway ({})", gateway));

    // Listen to messages
    let mut retry_count = 0;

    loop {
        if retry_count >= MAX_LISTEN_RETRIES {
            return Err("Listening failed after max retries".to_string());
        }

        match socket.read() {
            Ok(message) => {
                retry_count = 0;
                match message {
                    Message::Ping(msg) => handle_ping(msg, &mut socket),
                    Message::Close(_) => {
                        logger::log_warn("Gateway closed the connection".to_string());
                        return Err("Connection closed by gateway".to_string());
                    }
                    Message::Text(msg) => {
                        if let Ok(parsed) = serde_json::from_str::<ConnectAck>(&msg) {
                            handle_connect_ack(parsed);
                        } else if let Ok(parsed) = serde_json::from_str::<QueryCodebase>(&msg) {
                            handle_query_codebase(parsed, &mut socket);
                        } else {
                            handle_invalid_message(&mut socket);
                        }
                    }
                    Message::Binary(_) => {
                        logger::log_warn("Received unsupported binary message".to_string());
                    }
                    _ => {}
                }
            }
            Err(err) => {
                retry_count += 1;
                logger::log_warn(format!("Failed to read message: {} (retry attempt {} of {})", err, retry_count, MAX_LISTEN_RETRIES));
                std::thread::sleep(Duration::from_secs(1));
                let (new_socket, _) = connect(&gateway).map_err(|_| "Failed to reconnect gateway".to_string())?;
                socket = new_socket;
            }
        }
    }
}
