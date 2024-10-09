use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(long)]
    pub host: String,
    #[arg(long, default_value_t = 443)]
    pub port: u16,
    #[arg(long, value_name = "FILE")]
    pub cert: PathBuf,
    #[arg(long, value_name = "FILE")]
    pub key: PathBuf,
    #[arg(long, value_name = "FILE")]
    pub server_ca: PathBuf,
    #[arg(long)]
    pub target_sae_id: String,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Status,
    GetKeys {
        #[arg(long = "key-size", help = "Key size in bits", default_value_t = 256)]
        key_size_bits: u32,
        #[arg(
            long,
            help = "Additional SAE IDs allowed to retrieve the key",
            value_delimiter = ','
        )]
        allowed_sae_ids: Vec<String>,
        #[arg(long, help = "Amount of keys", default_value_t = 1)]
        amount: u32,
    },
    GetKeysByIds {
        #[arg(long, help = "Ids of keys to retrieve", value_delimiter = ',')]
        ids: Vec<String>,
    },
}
