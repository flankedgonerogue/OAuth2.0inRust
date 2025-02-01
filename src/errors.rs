use axum::response::{IntoResponse, Redirect, Response};
use urlencoding::encode;

fn create_error_response(
    redirect_uri: &String,
    error: &str,
    description: &str,
    state: Option<&String>,
) -> Response {
    let encoded_description = encode(description);
    let mut error_redirect_uri = format!(
        "{}?error={}&error_description={}",
        redirect_uri, error, encoded_description
    );

    if let Some(state) = state {
        error_redirect_uri.push_str("&state=");
        error_redirect_uri.push_str(state);
    }

    Redirect::permanent(&error_redirect_uri).into_response()
}

pub fn unsupported_response_type_error(redirect_uri: &String, state: Option<&String>) -> Response {
    create_error_response(
        redirect_uri,
        "unsupported_response_type",
        "The response_type parameter is missing or unsupported",
        state,
    )
}

pub fn invalid_client_error(redirect_uri: &String, state: Option<&String>) -> Response {
    create_error_response(
        redirect_uri,
        "invalid_client",
        "The client_id parameter is missing",
        state,
    )
}

pub fn missing_scope_error(redirect_uri: &String, state: Option<&String>) -> Response {
    create_error_response(
        redirect_uri,
        "invalid_scope",
        "The scope parameter is missing",
        state,
    )
}

pub fn invalid_scope_error(redirect_uri: &String, state: Option<&String>) -> Response {
    create_error_response(
        redirect_uri,
        "invalid_scope",
        "The scope parameter contains unknown scopes",
        state,
    )
}

pub fn database_error(redirect_uri: &String, state: Option<&String>) -> Response {
    create_error_response(
        redirect_uri,
        "server_error",
        "A database error occurred",
        state,
    )
}

pub fn invalid_redirect_uri_error(redirect_uri: &String, state: Option<&String>) -> Response {
    create_error_response(
        redirect_uri,
        "invalid_redirect_uri",
        "The redirect uri parameter is invalid",
        state,
    )
}

pub fn failed_authorization_error(redirect_uri: &String, state: Option<&String>) -> Response {
    create_error_response(
        redirect_uri,
        "access_denied",
        "The authorization request has been denied",
        state,
    )
}
