use crate::cache::add_client_and_request_to_cache;
use crate::database::get_client_data_in_db;
use crate::errors::{
    database_error, invalid_client_error, invalid_redirect_uri_error, invalid_scope_error,
};
use crate::pages::get_login_html;
use crate::storage::{check_client_id, AuthorizeRequestData};
use axum::response::{IntoResponse, Response};
use rand::random;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn authorization_code_flow(request_data: &AuthorizeRequestData) -> Response {
    // Check if client_id is valid
    if !check_client_id(&request_data.client_id).await {
        return invalid_client_error(&request_data.redirect_uri, request_data.state.as_ref());
    }

    // Get the client data
    let client_data = get_client_data_in_db(request_data.client_id.parse::<u32>().unwrap()).await;
    if client_data.is_none() {
        return database_error(&request_data.redirect_uri, request_data.state.as_ref());
    }
    let client_data = client_data.unwrap();

    // Match the redirect uri with allowed ones
    if !client_data
        .redirect_uris
        .contains(&request_data.redirect_uri)
    {
        return invalid_redirect_uri_error(&request_data.redirect_uri, request_data.state.as_ref());
    }

    // Match the requested scopes with allowed ones
    let scopes = request_data.scope.split(" ").collect::<Vec<&str>>();
    if !scopes
        .iter()
        .all(|s| client_data.allowed_scopes.contains(&s.to_string()))
    {
        return invalid_scope_error(&request_data.redirect_uri, request_data.state.as_ref());
    }

    // Generate a request id using a random and the current timestamp
    let request_id = generate_request_id();
    add_client_and_request_to_cache(&request_id, request_data, &client_data);

    get_login_html(client_data.name.as_str(), &request_id, &request_data.scope).into_response()
}

fn generate_request_id() -> String {
    // Get the current timestamp in seconds
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    // Generate a random 8-byte number
    let random_part: u64 = random();

    // Combine the timestamp and random part to form the request ID
    format!("{}-{:x}", timestamp, random_part)
}
