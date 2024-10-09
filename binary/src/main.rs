mod cli;

use crate::cli::Cli;
use crate::cli::Commands::{GetKeys, GetKeysByIds, Status};
use clap::Parser;
use etsi014_client::{ETSI014Client, Error, SecretVec};
use std::io;
use std::io::Write;
use std::process::exit;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    if let Err(e) = cli().await {
        eprintln!("{e}");
        exit(1)
    }
}

fn print_keys(keys: Vec<(String, SecretVec<u8>)>) {
    let keys_hex = keys
        .iter()
        .map(|(id, key)| {
            let key_hex = SecretVec::new(key.len() * 2, |s| {
                hex::encode_to_slice(key.borrow().as_ref(), s).unwrap()
            });
            (id, key_hex)
        })
        .collect::<Vec<_>>();
    keys_hex.iter().for_each(|(id, key_hex)| {
        print!("{id}=");
        io::stdout().write_all(key_hex.borrow().as_ref()).unwrap();
        println!();
    })
}

async fn cli() -> Result<(), Error> {
    let cli = Cli::parse();
    let client =
        ETSI014Client::new(&cli.host, cli.port, &cli.cert, &cli.key, &cli.server_ca)?;
    match cli.command {
        Status => {
            let s = client.get_status(&cli.target_sae_id).await?;
            Ok(println!(
                "source_KME_ID={}\n\
                target_KME_ID={}\n\
                source_SAE_ID={}\n\
                target_SAE_ID={}\n\
                key_size={}\n\
                stored_key_count={}\n\
                max_key_count={}\n\
                max_key_per_request={}\n\
                max_key_size={}\n\
                min_key_size={}\n\
                max_SAE_ID_count={}",
                s.source_kme_id,
                s.target_kme_id,
                s.source_sae_id,
                s.target_sae_id,
                s.key_size,
                s.stored_key_count,
                s.max_key_count,
                s.max_key_per_request,
                s.max_key_size,
                s.min_key_size,
                s.max_sae_id_count,
            ))
        }
        GetKeys {
            key_size_bits,
            allowed_sae_ids,
            amount,
        } => {
            let kl = client
                .get_keys(
                    key_size_bits,
                    &cli.target_sae_id,
                    &allowed_sae_ids
                        .iter()
                        .map(|a| a.as_ref())
                        .collect::<Vec<_>>(),
                    amount,
                )
                .await?;
            print_keys(kl);
            Ok(())
        }
        GetKeysByIds { ids } => {
            let kl = client
                .get_keys_by_ids(
                    &cli.target_sae_id,
                    &ids.iter().map(|a| a.as_ref()).collect::<Vec<_>>(),
                )
                .await?;
            print_keys(kl);
            Ok(())
        }
    }
}
