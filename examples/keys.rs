use api_builder::{api_rest_client, error::APIError, APIErrorKind, ReqwestClient, RestClient};
use luarmor::{
    LuarmorClient,
    models::{
        LuarmorMessage,
        v3::projects::users::{CreateUser, GetUsers, LinkDiscordId, ResetHwid, UpdateUser},
    },
};
use reqwest::{blocking::ClientBuilder, Proxy};

// Create our own client for sending requests
#[derive(Default, ReqwestClient)]
struct Client {
    client: reqwest::blocking::Client,
}
#[api_rest_client(error = LuarmorMessage, base = "\"https://api.luarmor.net\"")]
impl RestClient for Client {}

fn main() -> Result<(), APIError<LuarmorMessage>> {
    // Initialisation
    dotenv::dotenv().map_err(APIErrorKind::from_any_error)?;
    let api_key = std::env::var("API_KEY").map_err(APIErrorKind::from_any_error)?;
    let project_id = std::env::var("PROJECT_ID").map_err(APIErrorKind::from_any_error)?;

    // Construct the client, this one is proxy to Fiddler for testing
    println!("{api_key} - {project_id}");
    let client = LuarmorClient::new(api_key, Client {
        client: ClientBuilder::new()
            .proxy(Proxy::all("http://localhost:8888")?)
            .build()?
    });

    // Creating a key...
    let user_key = client.create_user(
        CreateUser::builder()
            .project_id(project_id.as_str())
            .identifier("test")
            .build(),
    )?;
    println!("Created key - {}", user_key);

    // See if we can find it
    let users = client.users(
        GetUsers::builder()
            .project_id(project_id.as_str())
            .user_key(user_key.as_str())
            .build(),
    )?;
    assert_eq!(users.len(), 1);
    println!("Found key!");

    // Set the discord
    let discord_id = "398271060514045964";
    client.link_discord(
        LinkDiscordId::builder()
            .project_id(project_id.as_str())
            .user_key(user_key.as_str())
            .discord_id(discord_id)
            .build(),
    )?;
    println!("Linked discord");

    // Reset HWID
    client.reset_hwid(
        ResetHwid::builder()
            .project_id(project_id.as_str())
            .user_key(user_key.as_str())
            .force(true)
            .build(),
    )?;
    println!("Reset hwid");

    // Update the key
    let note = "sigma";
    client.update_user(
        UpdateUser::builder()
            .project_id(project_id.as_str())
            .user_key(user_key.as_str())
            .note(note)
            .build(),
    )?;
    println!("Set note");

    // Grab the key again
    let users = client.users(
        GetUsers::builder()
            .project_id(project_id.as_str())
            .user_key(user_key.as_str())
            .build(),
    )?;
    assert_eq!(users.len(), 1);

    // Check if our changes were successful
    let user = users.first().unwrap();
    assert_eq!(user.note, Some(note.to_string()));
    assert_eq!(user.identifier, None);
    assert_eq!(user.discord_id, Some(discord_id.to_string()));

    println!("Found key and changes found!");

    // Finally delete the key
    client.delete_user(&project_id, &user_key)?;
    println!("Deleted key - {}", user_key);

    Ok(())
}
