use rocket::{post, serde::json::Json, State};
use std::str::FromStr;
use ethers::prelude::*;
use ethers::types::{U256, Address};
use crate::auth::structs::{AccountRequest, DeployResponse, AppState};
use crate::auth::utils::{der_key_to_contract_friendly_key, get_factory_client};



pub const SALT: U256 = U256::zero();


// Deploy new wallet ( createAccount )
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

    let (client, factory) = match get_factory_client(&state.factory_address, &state.private_key).await {
        Ok((client, factory)) =>( client, factory),
        Err(e) => {
            println!("Failed to get onchain client {}", e);

            return Json(DeployResponse {
                tx_hash: String::new(),
                status: "something happened".to_string(),
                address: String::new()
            });
        }
    };


    let wallet = match LocalWallet::from_str(&state.private_key) {
        Ok(w) => w,
        Err(e) => return Json(DeployResponse {
            tx_hash: String::new(),
            status: format!("Signer error: {}", e),
            address: String::new()
        }),
    };


    let initial_key_slot = request.initial_key_slot.unwrap_or(0);
    let from = wallet.address();

    println!("Salt: {}", SALT);

    // Predict address via call
    let get_address = factory
        .method::<_, Address>(
            "getAddress",
            (initial_key_slot, key_pair, SALT),
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
        .method::<_, Address>("createAccount", (initial_key_slot, key_pair, SALT))
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
