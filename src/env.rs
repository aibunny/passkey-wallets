use std::env;

fn initialize_env(){
    // Load .env
    dotenvy::dotenv().ok();
    println!("Loaded .env file successfully.");
}

pub fn get_env_var(env_name: &str) -> String {
    initialize_env();

    match env_name {
        "rpc" => env::var("RPC_URL").expect("rpc must be set in .env"),
        "entrypoint" => env::var("ENTRY_POINT_ADDRESS").expect("entrypoint must be set in .env"),
        "private_key" => env::var("PRIVATE_KEY").expect("private_key must be set in .env"),
        "account_factory" => env::var("FACTORY_ADDRESS").expect("entrypoint must be set in .env"),
        _ => panic!("Unsupported environment variable: {}", env_name),
    }

}


