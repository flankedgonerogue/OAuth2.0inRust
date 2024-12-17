use crate::cache::{add_client_id_to_cache, check_client_id_in_cache};
use crate::database::check_client_id_in_db;
use serde::{Deserialize, Serialize};

pub async fn check_client_id(client_id: &String) -> bool {
    println!("Checking for client id {} in cache", client_id);

    if check_client_id_in_cache(client_id) {
        println!("Found client id {} in cache", client_id);
        return true;
    }

    println!("Checking for client id {} in DB", client_id);

    if check_client_id_in_db(client_id.parse::<u32>().unwrap()).await {
        add_client_id_to_cache(client_id);
        println!("Found client id {} in database", client_id);
        return true;
    }

    false
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    pub id: u32,
    pub name: String,
    pub allowed_scopes: Vec<String>,
    pub redirect_uris: Vec<String>,
}

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
    pub password: String
}
