use crate::cache::get_client_and_request_to_cache;
use crate::errors::failed_authorization_error;
use crate::pages::get_login_error_html;
use crate::storage::LoginRequestData;
use axum::response::{IntoResponse, Response};
use std::collections::HashMap;
use axum::Form;

#[axum::debug_handler]
pub async fn serve_login(Form(params): Form<HashMap<String, String>>) -> Response {
    println!("params: {:?}", params);

    let request_data = match LoginRequestData::new(&params) {
        Some(data) => data,
        None => return get_login_error_html().into_response(),
    };

    let data = get_client_and_request_to_cache(&request_data.request_id);
    let request_data = data.0;
    let client_data = data.1;
    
    println!("request_data: {:?}", &request_data);
    println!("client_data: {:?}", &client_data);
    
    // TODO Complete the login in an authorization code gen
    // TODO Complete the code exchange for access and refresh token

    // Check if password matches, then perform a redirect
    failed_authorization_error(&request_data.redirect_uri, request_data.state.as_ref())
}
