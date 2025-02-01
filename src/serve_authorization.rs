use crate::errors::{invalid_client_error, missing_scope_error, unsupported_response_type_error};
use crate::flows::authorization_code_flow;
use crate::pages::get_error_html;
use crate::storage::AuthorizeRequestData;
use axum::extract::Query;
use axum::response::{IntoResponse, Response};
use regex::Regex;
use std::collections::HashMap;
use validator::{ValidateRegex, ValidateUrl};

#[axum::debug_handler]
pub async fn serve_authorization(Query(params): Query<HashMap<String, String>>) -> Response {
    // Create RequestData struct from query parameters
    let request_data = match AuthorizeRequestData::new(&params) {
        Some(data) => data,
        None => return get_error_html("Missing required parameters", "400").into_response(),
    };

    // Validate the redirect URI
    if !request_data.redirect_uri.validate_url() {
        return get_error_html("Invalid redirect URI", "400").into_response();
    }

    // Validate client ID
    if !request_data
        .client_id
        .validate_regex(Regex::new("^\\d+$").unwrap())
    {
        return invalid_client_error(&request_data.redirect_uri, request_data.state.as_ref());
    }

    // Validate scope
    if request_data.scope.is_empty() {
        return missing_scope_error(&request_data.redirect_uri, request_data.state.as_ref());
    }

    // Handle the flow based on the response_type
    match request_data.response_type.as_deref() {
        Some("code") => authorization_code_flow(&request_data).await,
        _ => {
            unsupported_response_type_error(&request_data.redirect_uri, request_data.state.as_ref())
        }
    }
}
