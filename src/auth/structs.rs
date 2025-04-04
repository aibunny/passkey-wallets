use serde::{ Serialize, Deserialize};



#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AccountRequest {
    pub public_key: String,
    pub initial_key_slot: Option<u8>,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DeployResponse {
    pub tx_hash: String,
    pub status: String,
    pub address: String,
}



#[derive(Debug, Serialize)]
pub struct AccountAddressResponse {
    pub address: String,
}


pub struct AppState {
    pub(crate) private_key: String,
    pub factory_address: String
}


unsafe impl Send for AppState {}
unsafe impl Sync for AppState {}