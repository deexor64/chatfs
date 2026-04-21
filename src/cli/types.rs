use std::path::PathBuf;
use clap::{Parser, Subcommand};

use crate::config::types::ConfigKey;

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(long, conflicts_with = "debug")]
    pub no_logging: bool,

    #[arg(long, conflicts_with = "no_logging")]
    pub debug: bool
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
        #[arg(value_name = "PATH")]
        workspace: Option<PathBuf>,

        #[arg(short, long)]
        gateway: Option<String>,
    }
}
