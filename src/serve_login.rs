use crate::errors::failed_authorization_error;
use crate::pages::get_login_error_html;
use crate::storage::LoginRequestData;
use crate::{GLOBAL_CACHE, GLOBAL_DATABASE};
use axum::response::{IntoResponse, Redirect, Response};
use axum::Form;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::collections::HashMap;

#[axum::debug_handler]
pub async fn serve_login(Form(params): Form<HashMap<String, String>>) -> Response {
    let form_data = match LoginRequestData::new(&params) {
        Some(data) => data,
        None => return get_login_error_html().into_response(),
    };

    let request_data = match GLOBAL_CACHE
        .get()
        .unwrap()
        .get_request(&form_data.request_id)
    {
        Some(data) => data,
        None => return get_login_error_html().into_response(),
    };

    let user = GLOBAL_DATABASE
        .get()
        .unwrap()
        .get_user(&form_data.email, &form_data.password)
        .await;
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

        GLOBAL_CACHE
            .get()
            .unwrap()
            .set_auth_code_for_user_and_scopes(
                &request_data.client_id.to_string(),
                &code,
                &user.id.to_string(),
                &request_data.scope,
            );

        // Return the auth code and state
        let redirect_uri = format!(
            "{}?code={}&state={}",
            request_data.redirect_uri,
            code,
            request_data.state.unwrap()
        );
        return Redirect::permanent(redirect_uri.as_str()).into_response();
    }

    failed_authorization_error(&request_data.redirect_uri, request_data.state.as_ref())
}
