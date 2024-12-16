use crate::get_redis_connection;
use crate::storage::{AuthorizeRequestData, Client};
use redis::Commands;

pub fn check_client_id_in_cache(client_id: &String) -> bool {
    let mut con = get_redis_connection();

    con.sismember("CLIENT_IDS", client_id).unwrap_or(false)
}

pub fn add_client_id_to_cache(client_id: &String) {
    let mut con = get_redis_connection();

    con.sadd("CLIENT_IDS", client_id).unwrap()
}

pub fn add_client_and_request_to_cache(
    request_id: &String,
    request_data: &AuthorizeRequestData,
    client: &Client,
) {
    let mut con = get_redis_connection();

    // Serialize request_data and client to JSON strings
    let request_data_json = serde_json::to_string(request_data).unwrap(); // Handle errors properly in real code
    let client_json = serde_json::to_string(client).unwrap(); // Handle errors properly in real code

    // Use serialized JSON strings as the values
    let _: () = con
        .set(
            format!("REQUEST_ID_{}_REQUEST_DATA", request_id),
            request_data_json,
        )
        .unwrap();
    let _: () = con
        .set(
            format!("REQUEST_ID_{}_CLIENT_DATA", request_id),
            client_json,
        )
        .unwrap();

    println!(
        "Saved request and client data for request id {}",
        request_id
    );
}

pub fn get_client_and_request_to_cache(request_id: &String) -> (AuthorizeRequestData, Client) {
    let mut con = get_redis_connection();

    // Use serialized JSON strings as the values
    let request_data: String = con
        .get(format!("REQUEST_ID_{}_REQUEST_DATA", request_id))
        .unwrap();
    let client_data: String = con
        .get(format!("REQUEST_ID_{}_CLIENT_DATA", request_id))
        .unwrap();

    println!("Got request and client data for request id {}", request_id);

    (
        serde_json::from_str(&request_data).unwrap(),
        serde_json::from_str(&client_data).unwrap(),
    )
}
