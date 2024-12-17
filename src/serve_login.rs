use crate::cache::{get_client_and_request_to_cache, store_auth_code_and_scope_to_cache};
use crate::errors::failed_authorization_error;
use crate::pages::get_login_error_html;
use crate::storage::LoginRequestData;
use axum::response::{IntoResponse, Redirect, Response};
use std::collections::HashMap;
use axum::Form;
use rand::distributions::Alphanumeric;
use rand::Rng;
use crate::database::find_user_in_db;

#[axum::debug_handler]
pub async fn serve_login(Form(params): Form<HashMap<String, String>>) -> Response {
    println!("params: {:?}", params);

    let form_data = match LoginRequestData::new(&params) {
        Some(data) => data,
        None => return get_login_error_html().into_response(),
    };

    let data = get_client_and_request_to_cache(&form_data.request_id);
    let request_data = data.0;
    let client_data = data.1;
    
    println!("request_data: {:?}", &request_data);
    println!("client_data: {:?}", &client_data);
    
    let user = find_user_in_db(&form_data.email, &form_data.password).await;
    if user.is_none() {
        failed_authorization_error(&request_data.redirect_uri, request_data.state.as_ref());
    }
    let user = user.unwrap();
    
    // User is authorized, generate a code
    if request_data.response_type.unwrap().eq("code") {
        // Generate a code and cache the user details etc. for access and refresh tokens
        let code: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32) 
            .map(char::from) 
            .collect();
        
        store_auth_code_and_scope_to_cache(&user.id, &client_data.id.to_string(), &code, &request_data.scope);
        
        // Return the auth code and state
        let redirect_uri = format!("{}?code={}&state={}", request_data.redirect_uri, code, request_data.state.unwrap());
        return Redirect::permanent(redirect_uri.as_str()).into_response()
    }
    
    failed_authorization_error(&request_data.redirect_uri, request_data.state.as_ref())
}
