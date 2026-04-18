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
    logger::log_info(format!("Connected to gateway ({})", gateway).to_string());

    // Listen to messages
    let mut retry_count = 0;

    loop {
        if retry_count >= MAX_LISTEN_RETRIES {
            return Err("Listening failed after max retries".to_string());
        }

        let message = socket.read();

        if let Err(_) = message {
            logger::log_warn(format!("Failed to read message (retrying...)").to_string());
            retry_count += 1;
            continue;
        }

        let message = message.unwrap(); // Won't panic here, becuase error case is already handled

        // Handle ping
        if let Message::Ping(msg) = message {
            handle_ping(msg, &mut socket);
        }
        // Handle other messages
        else if let Message::Text(msg) = message {
            if let Ok(parsed) =  serde_json::from_str::<ConnectAck>(&msg) {
                handle_connect_ack(parsed);

            } else if let Ok(parsed) =  serde_json::from_str::<QueryCodebase>(&msg) {
                handle_query_codebase(parsed, &mut socket);

            } else {
                handle_invalid_message(&mut socket);

            }
        }
    }
}
