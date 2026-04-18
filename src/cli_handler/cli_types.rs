use clap::{Parser, Subcommand};

use crate::tool_config::config_types::ConfigKey;

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>
}

#[derive(Subcommand)]
pub enum Commands {
    SetConfig {
        key: ConfigKey,
        value: String,
    },
    GetConfig {
        key: ConfigKey,
    },
    Run {
        #[arg(long)]
        no_logging: bool,

        #[arg(short, long)]
        gateway: Option<String>,
    }
}
