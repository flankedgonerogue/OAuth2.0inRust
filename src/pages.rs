use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "error.html")]

struct ErrorTemplate<'a> {
    message: &'a str,
    code: &'a str,
}

pub fn get_error_html<'a>(message: &'a str, code: &'a str) -> Html<String> {
    let html = ErrorTemplate { message, code };

    Html(
        html.render()
            .unwrap_or("An unknown error occurred".to_string()),
    )
}

#[derive(Template)]
#[template(path = "login.html")]

struct LoginTemplate<'a> {
    client_name: &'a str,
    request_id: &'a str,
    scope_list: &'a str,
}

pub fn get_login_html<'a>(
    client_name: &'a str,
    request_id: &'a str,
    scope_list: &'a str,
) -> Html<String> {
    let html = LoginTemplate {
        client_name,
        request_id,
        scope_list,
    };

    Html(
        html.render()
            .unwrap_or("An unknown error occurred".to_string()),
    )
}

#[derive(Template)]
#[template(path = "login-error.html")]

struct LoginError {}

pub fn get_login_error_html<'a>() -> Html<String> {
    Html(
        LoginError {}
            .render()
            .unwrap_or("An unknown error occurred".to_string()),
    )
}
