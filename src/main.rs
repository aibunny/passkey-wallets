mod env;
mod auth;


use rocket::{self, routes};
use crate::auth::services::{deploy};
use crate::auth::structs::AppState;
use crate::env::get_env_var;



#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    let private_key = get_env_var("private_key");
    let factory_address = get_env_var("account_factory");

    let _rocket = rocket::build()
        .mount("/", routes![deploy])
        .manage(AppState {
            private_key,
            factory_address
        })
        .launch()
        .await?;

    Ok(())
}