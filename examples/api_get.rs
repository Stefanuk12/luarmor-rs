// Dependencies
use luarmor::client::Client;

/// Entrypoint.
#[tokio::main]
async fn main() {
    // Load any `.env` variables.
    dotenv::dotenv().unwrap();

    // Make sure we are given an API key
    let api_key = std::env::var("API_KEY").expect("expected `API_KEY` in the environment.");

    // Construct the client
    let client = Client::new(&api_key);

    // Log each request
    println!("API status:\n{:?}\n---", client.status().await.expect("status failed"));
    println!("API key details:\n{:?}\n---", client.key_details().await.expect("key_details failed"));
    println!("API Key stats:\n{:?}\n---", client.key_stats(false).await.expect("key_stats failed"));
}