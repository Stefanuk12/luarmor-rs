// Dependencies
use luarmor::{client::Client, api_models::{CreatePayload, APIResponse, GetPayload, LinkDiscordPayload, ResetHWIDPayload, EditPayload}};

/// Entrypoint.
#[tokio::main]
async fn main() {
    // Load any `.env` variables.
    dotenv::dotenv().unwrap();

    // Make sure we are given an API key and project
    let api_key = std::env::var("API_KEY").expect("expected `API_KEY` in the environment.");
    let project_id = &std::env::var("PROJECT_ID").expect("expected `PROJECT_ID` in the environment.");

    // Construct the client
    let client = Client::new(&api_key);

    // Creating a key...
    let APIResponse::Success(result) = client.create_key(project_id, &CreatePayload {
        identifier: Some("test".to_string()),
        ..Default::default()
    }).await.expect("create_key failed") else { panic!("create_key failed (2)") };
    println!("Created key - {}", result.user_key.clone());

    // See if we can find it
    let APIResponse::Success(users) = client.get_keys(project_id, &GetPayload {
        user_key: Some(result.user_key.clone()),
        ..Default::default()
    }).await.expect("get_keys failed") else { panic!("get_keys failed (2)") };
    assert_eq!(users.users.len(), 1);
    println!("Found key!");

    // Set the discord and reset HWID
    let set_field = String::from("398271060514045964");
    let APIResponse::Success(_discord) = client.link_discord(project_id, &LinkDiscordPayload {
        user_key: result.user_key.clone(),
        discord_id: set_field.clone(),
        ..Default::default()
    }).await.expect("link_discord failed") else { panic!("link_discord failed (2)") };
    println!("Reset discord");
    let APIResponse::Success(_identifier) = client.reset_hwid(project_id, &ResetHWIDPayload {
        user_key: result.user_key.clone(),
        force: Some(true)
    }).await.expect("reset_hwid failed") else { panic!("reset_hwid failed (2)") };
    println!("Reset hwid");

    // Update the key
    let APIResponse::Success(_edit) = client.update_key(project_id, &EditPayload {
        user_key: result.user_key.clone(),
        note: Some(set_field.clone()),
        ..Default::default()
    }).await.expect("update_key failed") else { panic!("upate_key failed (2)")};
    println!("Set note");

    // Grab the key again
    let APIResponse::Success(users) = client.get_keys(project_id, &GetPayload {
        user_key: Some(result.user_key.clone()),
        ..Default::default()
    }).await.expect("get_keys failed") else { panic!("get_keys failed (2)") };

    assert_eq!(users.users.len(), 1);
    let user = users.users.first().unwrap();
    assert_eq!(user.note, set_field.clone());
    assert_eq!(user.identifier, String::from(""));
    assert_eq!(user.discord_id, set_field.clone());

    println!("Found key and changes found!");

    // Finally delete the key
    let APIResponse::Success(_delete) = client.delete_key(project_id, &result.user_key.clone()).await.expect("get_keys failed") else { panic!("get_keys failed (2)") };
    println!("Deleted key - {}", user.user_key);
}