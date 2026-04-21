use serde::{Deserialize, Serialize};
use clap::{ValueEnum};


#[derive(Clone, ValueEnum)]
pub enum ConfigKey {
    Gateway,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub gateway: String,
}
