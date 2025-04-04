use rocket::{post, serde::json::Json, State};
use std::sync::Arc;
use std::str::FromStr;
use ethers::prelude::*;
use ethers::types::{U256, Address};
use crate::auth::contract::{AccountFactory};
use crate::auth::structs::{AccountRequest, DeployResponse, AppState};
use crate::auth::utils::{der_key_to_contract_friendly_key};
use crate::env::get_env_var;



#[post("/account/deploy", format = "json", data = "<request>")]
pub async fn deploy(request: Json<AccountRequest>, state: &State<AppState>) -> Json<DeployResponse> {
    let key_pair = match der_key_to_contract_friendly_key(&request.public_key) {
        Ok(k) => k,
        Err(e) => return Json(DeployResponse {
            tx_hash: String::new(),
            status: format!("Failed to process key: {}", e),
            address: String::new()
        }),
    };

    let provider_url = get_env_var("rpc");
    let provider = match Provider::<Http>::try_from(provider_url) {
        Ok(p) => p,
        Err(e) => return Json(DeployResponse {
            tx_hash: String::new(),
            status: format!("Provider error: {}", e),
            address: String::new()
        }),
    };



    let wallet = match LocalWallet::from_str(&state.private_key) {
        Ok(w) => w,
        Err(e) => return Json(DeployResponse {
            tx_hash: String::new(),
            status: format!("Signer error: {}", e),
            address: String::new()
        }),
    };

    let chain_id = match provider.get_chainid().await {
        Ok(id) => id.as_u64(),
        Err(e) => return Json(DeployResponse {
            tx_hash: String::new(),
            status: format!("Chain ID error: {}", e),
            address: String::new()
        }),
    };

    let wallet = wallet.with_chain_id(chain_id);

    println!("the wallet {} and chain {}", wallet.address(), chain_id);

    let client = Arc::new(SignerMiddleware::new(provider, wallet.clone()));

    let factory_address = match Address::from_str(&state.factory_address) {
        Ok(addr) => addr,
        Err(e) => return Json(DeployResponse {
            tx_hash: String::new(),
            status: format!("Invalid factory address: {}", e),
            address: String::new()
        }),
    };

    let factory = AccountFactory::new(factory_address, client.clone());

    let initial_key_slot = request.initial_key_slot.unwrap_or(0);
    let salt = U256::zero();
    let from = wallet.address();

    println!("Salt: {}", salt);

    // Predict address via call
    let get_address = factory
        .method::<_, Address>(
            "getAddress",
            (initial_key_slot, key_pair, salt),
        )
        .unwrap()
        .call()
        .await;

    let predicted_address = match get_address {
        Ok(addr) => addr,
        Err(e) => return Json(DeployResponse {
            tx_hash: String::new(),
            status: format!("Error calling getAddress: {}", e),
            address: String::new()
        }),
    };

    println!("Predicted address: {}", format!("{:#x}", predicted_address));
    println!("Signer address: {}", format!("{:#x}", from));

    // Check if contract is already deployed
    let code = match client.get_code(predicted_address, None).await {
        Ok(code) => code,
        Err(e) => return Json(DeployResponse {
            tx_hash: String::new(),
            status: format!("Error checking code: {}", e),
            address: String::new()
        }),
    };

    if !code.0.is_empty() {
        return Json(DeployResponse {
            tx_hash: String::new(),
            status: "Contract already exists".to_string(),
            address: format!("{:#x}", predicted_address),
        });
    }

    let max = U256::from(100_000_000_000u64); // 100 Gwei

    let call = factory
        .method::<_, Address>("createAccount", (initial_key_slot, key_pair, salt))
        .unwrap()
        .value(U256::from(10u64.pow(16)))
        .from(from)
        .gas_price(max);

    let tx = call.send().await;

    match tx {
        Ok(pending_tx) => {
            let tx_hash = format!("{:#x}", pending_tx.tx_hash());

            Json(DeployResponse {
                tx_hash,
                status: "success".to_string(),
                address: format!("{:#x}", predicted_address)
            })
        }
        Err(e) => Json(DeployResponse {
            tx_hash: String::new(),
            status: format!("Error sending tx: {}", e),
            address: String::new()
        }),
    }
}
