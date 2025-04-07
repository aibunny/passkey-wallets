use std::sync::Arc;
use ethers::types::H256;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use ethers::middleware::SignerMiddleware;
use ethers::prelude::*;
use ethers::prelude::{Http, LocalWallet, Provider};
use ethers::types::{Address};
use std::error::Error;
use crate::auth::contract::AccountFactory;
use crate::env::get_env_var;
use std::str::FromStr;

pub fn der_key_to_contract_friendly_key(public_key_b64url: &str) -> Result<[H256; 2], String> {
    let der_bytes = URL_SAFE
        .decode(public_key_b64url)
        .map_err(|e| format!("Base64URL decode error: {}", e))?;

    if der_bytes.len() < 65 {
        return Err(format!("DER too short, got {} bytes", der_bytes.len()));
    }

    let ec_point = &der_bytes[der_bytes.len() - 65..];

    if ec_point[0] != 0x04 {
        return Err("Expected EC point to start with 0x04".to_string());
    }

    let x = H256::from_slice(&ec_point[1..33]);
    let y = H256::from_slice(&ec_point[33..65]);

    println!("x: {}", x);
    println!("y: {}", y);

    Ok([x, y])
}


pub async fn get_factory_client(
    factory_address: &str,
    private_key: &str
) -> Result<(
    Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
    AccountFactory<SignerMiddleware<Provider<Http>, LocalWallet>>
), Box<dyn Error>> {
    let provider_url = get_env_var("rpc");
    let provider = Provider::<Http>::try_from(provider_url)?;

    let chain_id = provider.get_chainid().await?;

    let wallet = LocalWallet::from_str(private_key)?.with_chain_id(chain_id.as_u64());

    println!("the wallet {} and chain {:?}", wallet.address(), chain_id);

    let client = Arc::new(SignerMiddleware::new(provider, wallet.clone()));

    let factory_address = Address::from_str(factory_address)?;
    let factory = AccountFactory::new(factory_address, client.clone());

    Ok((client, factory))
}

