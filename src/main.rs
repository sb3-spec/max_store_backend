use models::db::connect_to_db;
use std::{env, sync::Arc};

mod models;

const DEFAULT_WEB_PORT: u16 = 8080;
#[tokio::main]
async fn main() {
    match dotenvy::dotenv() {
        Ok(_) => println!("Dev vars successfully loaded"),
        Err(_) => println!("Failed to load dev vars"),
    };

    let web_port: u16 = match env::var("PORT") {
        Ok(port) => port.parse::<u16>().unwrap(),
        Err(_) => DEFAULT_WEB_PORT,
    };

    // Connect to database
    let db = Arc::new(connect_to_db().await.expect("Cannot connect to db"));
}
