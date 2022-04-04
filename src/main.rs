use std::error::Error;

use comrade::client::Client;
use tracing::{error};


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize dotenv
    dotenv::dotenv().expect("Failed to load .env file");

    // Initialize the logger to use environment variables.
    tracing_subscriber::fmt::init();

    let mut client = Client::default().await?;

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
    Ok(())
}