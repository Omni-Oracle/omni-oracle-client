use anchor_client::anchor_lang::prelude::*;
use anchor_client::Program;
use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::{solana_sdk::{
    signature::{read_keypair_file, Keypair, Signer},
    system_program,
},
Client, Cluster, ClientError};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use omni_oracle::{Reputation, SetReputation, UpdatePrice, InitializeAsset, CustomError, Asset};

pub async fn initialize_asset(
    program: &Program<Rc<Keypair>>,
    assetId: Pubkey,
    metadata_url: String,
) -> std::result::Result<(), ClientError> {
    
    let authority = Keypair::from_base58_string("");
    let (asset_pda, _bump) = Pubkey::find_program_address(&[b"OMNI".as_ref(), assetId.as_ref()], &program.id());

    let tx = program
        .request()
        .accounts(omni_oracle::accounts::InitializeAsset {
            asset: asset_pda,
            authority: authority.pubkey(),
            system_program: system_program::ID,
        })
        .args(omni_oracle::instruction::InitializeAsset {
            assetId,
            metadata_url,
        })
        .signer(&authority)
        .send()
        .await;

        match tx {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into()), // Convert the error to a compatible type
        }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Create client
    let payer = Keypair::from_base58_string("");
    let client = Client::new(Cluster::Devnet, Rc::new(payer));

    // Create program
    let program = client.program(omni_oracle::ID).unwrap();
    
    let my_account_kp = Keypair::new();

    let sig = initialize_asset(&program, my_account_kp.pubkey(), "https://example.com/metadata.json".to_string()).await;
    print!("{:?}", sig);
    Ok(())
}
