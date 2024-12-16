use axum::response::{IntoResponse, Redirect, Response};

pub fn unsupported_response_type_error(redirect_uri: &String, state: Option<&String>) -> Response {
    let mut error_redirect_uri = format!("{}?error=unsupported_response_type", redirect_uri);
    error_redirect_uri.push_str(
        "&error_description=The%20response_type%20parameter%20is%20missing%20or%20unsupported",
    );

    if let Some(state) = state {
        error_redirect_uri.push_str("&state=");
        error_redirect_uri.push_str(state);
    }

    Redirect::permanent(&error_redirect_uri).into_response()
}

pub fn invalid_client_error(redirect_uri: &String, state: Option<&String>) -> Response {
    let mut error_redirect_uri = format!("{}?error=invalid_client", redirect_uri);
    error_redirect_uri.push_str("&error_description=The%20client_id%20parameter%20is%20missing");

    if let Some(state) = state {
        error_redirect_uri.push_str("&state=");
        error_redirect_uri.push_str(state);
    }

    Redirect::permanent(&error_redirect_uri).into_response()
}

pub fn missing_scope_error(redirect_uri: &String, state: Option<&String>) -> Response {
    let mut error_redirect_uri = format!("{}?error=invalid_scope", redirect_uri);
    error_redirect_uri.push_str("&error_description=The%20scope%20parameter%20is%20missing");

    if let Some(state) = state {
        error_redirect_uri.push_str("&state=");
        error_redirect_uri.push_str(state);
    }

    Redirect::permanent(&error_redirect_uri).into_response()
}

pub fn invalid_scope_error(redirect_uri: &String, state: Option<&String>) -> Response {
    let mut error_redirect_uri = format!("{}?error=invalid_scope", redirect_uri);
    error_redirect_uri
        .push_str("&error_description=The%20scope%20parameter%20contains%20unknown%20scopes");

    if let Some(state) = state {
        error_redirect_uri.push_str("&state=");
        error_redirect_uri.push_str(state);
    }

    Redirect::permanent(&error_redirect_uri).into_response()
}

pub fn database_error(redirect_uri: &String, state: Option<&String>) -> Response {
    let mut error_redirect_uri = format!("{}?error=server_error", redirect_uri);
    error_redirect_uri.push_str("&error_description=A%20database%20error%20occurred");

    if let Some(state) = state {
        error_redirect_uri.push_str("&state=");
        error_redirect_uri.push_str(state);
    }

    Redirect::permanent(&error_redirect_uri).into_response()
}

pub fn invalid_redirect_uri_error(redirect_uri: &String, state: Option<&String>) -> Response {
    let mut error_redirect_uri = format!("{}?error=invalid_redirect_uri", redirect_uri);
    error_redirect_uri.push_str("&error_description=The%redirect%20uri%20parameter%20is%20invalid");

    if let Some(state) = state {
        error_redirect_uri.push_str("&state=");
        error_redirect_uri.push_str(state);
    }

    Redirect::permanent(&error_redirect_uri).into_response()
}

pub fn failed_authorization_error(redirect_uri: &String, state: Option<&String>) -> Response {
    let mut error_redirect_uri = format!("{}?error=access_denied", redirect_uri);
    error_redirect_uri
        .push_str("&error_description=The%20authorization%20request%20has%20been%20denied");

    if let Some(state) = state {
        error_redirect_uri.push_str("&state=");
        error_redirect_uri.push_str(state);
    }

    Redirect::permanent(&error_redirect_uri).into_response()
}
