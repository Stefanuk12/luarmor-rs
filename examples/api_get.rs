use api_builder::{api_rest_client, error::APIError, ReqwestClient, RestClient};
use luarmor::{models::LuarmorMessage, LuarmorClient};

// Create our own client for sending requests
#[derive(Default, ReqwestClient)]
struct Client {
    client: reqwest::blocking::Client,
}
#[api_rest_client(error = LuarmorMessage, base = "\"https://api.luarmor.net\"")]
impl RestClient for Client { }

fn main() -> Result<(), APIError<LuarmorMessage>> {
    // Initialisation
    dotenv::dotenv().map_err(APIError::from_any_error)?;
    let api_key = std::env::var("API_KEY").map_err(APIError::from_any_error)?;

    // Construct the client
    let client = LuarmorClient::new(api_key, Client::default());

    // Log each request
    println!("API status:\n{:?}\n---", client.status()?);
    println!("API key details:\n{:?}\n---", client.details()?);
    println!("API Key stats:\n{:?}\n---", client.stats(false)?);
    Ok(())
}