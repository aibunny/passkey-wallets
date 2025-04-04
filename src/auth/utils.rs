use ethers::types::H256;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};



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
