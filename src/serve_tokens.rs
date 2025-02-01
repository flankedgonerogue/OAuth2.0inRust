use crate::storage::{check_client_id, get_client_data, Client};
use crate::GLOBAL_CACHE;
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use jsonwebtoken::{encode, EncodingKey, Header};
use log::{debug, error};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

// Request body for the token exchange
#[derive(Deserialize)]
pub struct TokenRequest {
    client_id: String,
    client_secret: String,
    auth_code: String,
}

// Response body containing the access token
#[derive(Serialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
}

// JWT Claims
#[derive(Serialize)]
struct Claims {
    sub: String, // Subject (user ID)
    client_id: String,
    scopes: String,
    exp: usize, // Expiration timestamp
}

/// The main function that handles the token exchange.
pub async fn serve_tokens(Json(payload): Json<TokenRequest>) -> impl IntoResponse {
    let TokenRequest {
        client_id,
        client_secret,
        auth_code,
    } = payload;

    debug!("Token request received for client_id: {}", client_id);

    // Validate the client ID and secret
    if !check_client_id(client_id.as_str()).await {
        error!("Invalid client ID: {}", client_id);
        return (StatusCode::BAD_REQUEST, "Invalid client_id").into_response();
    }

    let client_data: Option<Client> = get_client_data(client_id.as_str()).await;

    if let Some(client) = client_data {
        if client.secret != client_secret {
            error!("Invalid client secret for client_id: {}", client_id);
            return (StatusCode::UNAUTHORIZED, "Invalid client_secret").into_response();
        }
    } else {
        error!(
            "Client data not found in cache for client_id: {}",
            client_id
        );
        return (StatusCode::BAD_REQUEST, "Client data not found").into_response();
    }

    // Get the cached auth code and scopes
    let (user_id, scopes) = match GLOBAL_CACHE
        .get()
        .unwrap()
        .get_auth_code_and_scopes(client_id.as_str(), auth_code.as_str())
    {
        (Some(user_id), Some(scopes)) => (user_id, scopes),
        (Some(_), None) => {
            return (StatusCode::UNAUTHORIZED, "Unknown scopes for auth code").into_response()
        }
        (None, Some(_)) => {
            return (StatusCode::UNAUTHORIZED, "Unknown user id for auth code").into_response()
        }
        (None, None) => return (StatusCode::UNAUTHORIZED, "Unknown auth code").into_response(),
    };

    // Generate JWT
    let expiration_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
        + 3600; // Expires in 1 hour

    let claims = Claims {
        sub: user_id.to_string(),
        client_id: client_id.clone(),
        scopes: scopes.clone(),
        exp: expiration_time as usize,
    };

    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| {
        error!("JWT_SECRET environment variable is not set");
        panic!("JWT_SECRET must be set");
    });

    let encoding_key = EncodingKey::from_secret(jwt_secret.as_bytes());
    let token = encode(&Header::default(), &claims, &encoding_key).unwrap_or_else(|err| {
        error!("Failed to generate JWT: {}", err);
        String::new()
    });

    if token.is_empty() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to generate access_token",
        )
            .into_response();
    }

    // Return the response
    let response = TokenResponse {
        access_token: token,
        token_type: "Bearer".to_string(),
        expires_in: 3600,
    };

    (StatusCode::OK, Json(response)).into_response()
}
