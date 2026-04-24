use std::sync::OnceLock;
use std::time::Duration;
use tungstenite::{Message, connect};

use super::types::{ConnectSyn, Ping, LlmCommand};
use super::handlers::{handle_connect_syn, handle_ping, handle_llm_command};
use crate::utils::logger;

static GATEWAY: OnceLock<String> = OnceLock::new();

pub fn set_gateway(url: String) {
    GATEWAY.set(url).unwrap();
}

pub fn get_gateway() -> Option<String> {
    GATEWAY.get().cloned()
}

const MAX_LISTEN_RETRIES: usize = 5;

pub fn socket_loop() -> Result<(), String> {
    // Gateway not set
    let gateway = get_gateway().unwrap_or("".to_string());

    if gateway.is_empty() {
        logger::log_info("Gateway url is not set or an empty url is provided (use 'config-set gateway <URL>' or '--gateway <URL>' to set)".to_string());
        return Ok(());
    }

    // Connect to socket
    let (mut socket, _response) = connect(&gateway).map_err(|_| "Failed to connect gateway (check gateway url using 'config-get gateway')".to_string())?;
    logger::log_debug("Connected to gateway".to_string());
    logger::log_debug("Started sharing workspace".to_string());

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
                    Message::Close(_) => {
                        logger::log_warn("Connection closed by gateway".to_string());
                        return Ok(());
                    },
                    Message::Ping(msg) => {
                        if let Err(e) = socket.write(Message::Pong(msg)) {
                            logger::log_error(e.to_string());
                        }
                    },
                    Message::Text(msg) => {
                        if let Ok(parsed) = serde_json::from_str::<ConnectSyn>(&msg) {
                            if let Err(e) = handle_connect_syn(parsed, &mut socket) {
                                logger::log_error(e);
                            }

                        } else if let Ok(_) = serde_json::from_str::<Ping>(&msg) {
                            // This is a manual ping/pong message to manage the life cycle of client objects at server
                            if let Err(e) = handle_ping(&mut socket) {
                                logger::log_error(e);
                            }

                        } else if let Ok(parsed) = serde_json::from_str::<LlmCommand>(&msg) {
                            if let Err(e) = handle_llm_command(parsed, &mut socket) {
                                logger::log_error(e);
                            }

                        } else {
                            logger::log_warn("Invalid message ignored".to_string());

                        }
                    },
                    Message::Binary(_) => {
                        logger::log_warn("Unsupported binary message ignored".to_string());
                    },
                    _ => {}
                }
            },
            Err(err) => {
                retry_count += 1;
                logger::log_warn(format!("Failed to read message: {} (retry attempt {} of {})", err, retry_count, MAX_LISTEN_RETRIES));

                // Wait before retrying
                std::thread::sleep(Duration::from_secs(1));
                let (new_socket, _) = connect(&gateway).map_err(|_| "Failed to reconnect gateway".to_string())?;

                // Update socket
                socket = new_socket;
            }
        }
    }
}
