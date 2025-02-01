pub(crate) mod cache;
pub(crate) mod database;

use serde::{Deserialize, Serialize};

pub async fn check_client_id(client_id: &str) -> bool {
    debug!("Checking for client ID {} in cache", client_id);

    // Check cache first
    if let Some(cache) = GLOBAL_CACHE.get() {
        if cache.is_client_id_present(client_id) {
            debug!("Found client ID {} in cache", client_id);
            return true;
        }
    } else {
        error!("Cache is not initialized!");
    }

    debug!("Client ID {} not in cache. Checking database...", client_id);

    // Parse client_id to u32
    let client_id_parsed = match client_id.parse::<u32>() {
        Ok(id) => id,
        Err(_) => {
            error!("Invalid client ID format: {}", client_id);
            return false;
        }
    };

    // Query database
    if let Some(database) = GLOBAL_DATABASE.get() {
        if database.find_client(&client_id_parsed).await {
            // Add to cache if found in the database
            if let Some(cache) = GLOBAL_CACHE.get() {
                cache.add_client_id(client_id);
                debug!("Found client ID {} in database", client_id);
            }
            return true;
        }
    } else {
        error!("Database is not initialized!");
    }

    debug!("Client ID {} not found", client_id);
    false
}

pub async fn get_client_data(client_id: &str) -> Option<Client> {
    let data_from_cache = GLOBAL_CACHE.get().unwrap().get_client(client_id);
    if let Some(data) = data_from_cache {
        Some(data)
    } else {
        let data = GLOBAL_DATABASE
            .get()
            .unwrap()
            .get_client(&client_id.parse::<u32>().unwrap())
            .await;
        if let Some(data) = data {
            GLOBAL_CACHE.get().unwrap().set_client(&data);
            Some(data)
        } else {
            None
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    pub id: u32,
    pub name: String,
    pub allowed_scopes: Vec<String>,
    pub redirect_uris: Vec<String>,
    pub secret: String,
}

use crate::{GLOBAL_CACHE, GLOBAL_DATABASE};
use log::{debug, error};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizeRequestData {
    pub client_id: String,
    pub redirect_uri: String,
    pub scope: String,
    pub state: Option<String>,
    pub response_type: Option<String>,
}

impl<'a> AuthorizeRequestData {
    pub fn new(params: &'a HashMap<String, String>) -> Option<Self> {
        let client_id = params.get("client_id")?.clone();
        let redirect_uri = params.get("redirect_uri")?.clone();
        let scope = params.get("scope")?.clone();
        let state = params.get("state").cloned();
        let response_type = params.get("response_type").cloned();

        Some(AuthorizeRequestData {
            client_id,
            redirect_uri,
            scope,
            state,
            response_type,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequestData {
    pub request_id: String,
    pub email: String,
    pub password: String,
}

impl<'a> LoginRequestData {
    pub fn new(params: &'a HashMap<String, String>) -> Option<Self> {
        let request_id = params.get("request_id")?.clone();
        let email = params.get("email")?.clone();
        let password = params.get("password")?.clone();

        Some(LoginRequestData {
            request_id,
            email,
            password,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub email: String,
    pub password: String,
}
