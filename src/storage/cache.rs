use crate::storage::{AuthorizeRequestData, Client};
use log::{debug, error, warn};
use redis::{Client as RedisClient, Commands};
use serde_json;

#[derive(Debug)]
pub struct Cache {
    client: RedisClient,
    prefix: String,
}

impl Cache {
    /// Create a new Cache instance with a Redis connection and a custom prefix.
    pub fn new(client: RedisClient, prefix: String) -> Self {
        Cache { client, prefix }
    }

    // Helper function to get a Redis connection
    fn get_connection(&self) -> redis::Connection {
        self.client.get_connection().unwrap_or_else(|err| {
            panic!("Failed to get Redis connection: {}", err);
        })
    }

    // Helper function to get the full Redis key with prefix
    fn get_prefixed_key(&self, key: &str) -> String {
        format!("{}:{}", self.prefix, key)
    }

    pub(super) fn is_client_id_present(&self, client_id: &str) -> bool {
        let mut con = self.get_connection();
        con.sismember(self.get_prefixed_key("CLIENT_IDS"), client_id)
            .unwrap_or_else(|err| {
                error!("Failed to check client ID in cache: {}", err);
                false
            })
    }

    pub(super) fn add_client_id(&self, client_id: &str) {
        let mut con = self.get_connection();
        con.sadd(self.get_prefixed_key("CLIENT_IDS"), client_id)
            .unwrap_or_else(|err| {
                error!("Failed to add client ID to cache: {}", err);
            });
    }

    pub fn set_request(&self, request_id: &str, request_data: &AuthorizeRequestData) {
        let mut con = self.get_connection();

        let request_data_json = serde_json::to_string(request_data).unwrap_or_else(|err| {
            error!("Failed to serialize request data: {}", err);
            String::new()
        });

        if !request_data_json.is_empty() {
            con.set_ex(
                self.get_prefixed_key(&format!("REQUEST_ID_{}_REQUEST_DATA", request_id)),
                request_data_json,
                600,
            )
            .unwrap_or_else(|err| {
                error!("Failed to store request data in cache: {}", err);
            });

            debug!(
                "Saved request and client data for request ID {}",
                request_id
            );
        }
    }

    pub fn get_request(&self, request_id: &str) -> Option<AuthorizeRequestData> {
        let mut con = self.get_connection();

        let request_data: Option<String> = con
            .get(self.get_prefixed_key(&format!("REQUEST_ID_{}_REQUEST_DATA", request_id)))
            .unwrap_or_else(|err| {
                warn!("Failed to retrieve request data from cache: {}", err);
                None
            });

        if let Some(request_json) = request_data {
            let request: AuthorizeRequestData =
                serde_json::from_str(&request_json).unwrap_or_else(|err| {
                    error!("Failed to deserialize request data: {}", err);
                    panic!("Corrupted cache data");
                });

            debug!("Retrieved request for request ID {}", request_id);
            Some(request)
        } else {
            warn!("Failed to retrieve data for request ID {}", request_id);
            None
        }
    }

    pub(super) fn set_client(self: &Self, client: &Client) {
        let mut con = self.get_connection();

        let client_json = serde_json::to_string(client).unwrap_or_else(|err| {
            error!("Failed to serialize client data: {}", err);
            String::new()
        });

        if !client_json.is_empty() {
            warn!("Failed to save client {}", client.id);
        }

        con.set_ex(
            self.get_prefixed_key(&format!("CLIENT_{}_DATA", client.id)),
            client_json,
            600,
        )
        .unwrap_or_else(|err| {
            warn!(
                "Failed to store client {} data in cache: {}",
                client.id, err
            );
        });

        debug!("Stored client {} data in cache", client.id);
    }

    pub(super) fn get_client(self: &Self, client_id: &str) -> Option<Client> {
        let mut con = self.get_connection();

        let client_data: Option<String> = con
            .get(self.get_prefixed_key(&format!("CLIENT_{}_DATA", client_id)))
            .unwrap_or_else(|err| {
                warn!("Failed to retrieve client data from cache: {}", err);
                None
            });

        if let Some(data) = client_data {
            let client: Client = serde_json::from_str(&data).unwrap_or_else(|err| {
                error!("Failed to deserialize client data: {}", err);
                panic!("Corrupted cache data");
            });

            Some(client)
        } else {
            warn!("Failed to retrieve data for client ID {}", client_id);
            None
        }
    }

    pub fn set_auth_code_for_user_and_scopes(
        &self,
        client_id: &str,
        code: &str,
        user_id: &str,
        scopes: &str,
    ) {
        let mut con = self.get_connection();

        con.set_ex(
            self.get_prefixed_key(&format!("AUTH_CLIENT_{}_CODE_{}_SCOPES", client_id, code)),
            scopes.to_string(),
            600,
        )
        .unwrap_or_else(|err| {
            error!("Failed to store scopes in cache: {}", err);
        });

        con.set_ex(
            self.get_prefixed_key(&format!("AUTH_CLIENT_{}_CODE_{}_USER_ID", client_id, code)),
            user_id.to_string(),
            600,
        )
        .unwrap_or_else(|err| {
            error!("Failed to store user id in cache: {}", err);
        });

        debug!(
            "Saved auth client {} code {} user id and scopes",
            client_id, code
        );
    }

    pub fn get_auth_code_and_scopes(
        &self,
        client_id: &str,
        code: &str,
    ) -> (Option<String>, Option<String>) {
        let mut con = self.get_connection();

        let user_id: Option<String> = con
            .get(self.get_prefixed_key(&format!("AUTH_CLIENT_{}_CODE_{}_USER_ID", client_id, code)))
            .unwrap_or_else(|_| {
                warn!("Failed to retrieve user_id from cache: {}", code);
                None
            });

        let scopes: Option<String> = con
            .get(self.get_prefixed_key(&format!("AUTH_CLIENT_{}_CODE_{}_SCOPES", client_id, code)))
            .unwrap_or_else(|_| {
                warn!("Failed to retrieve scope from cache: {}", code);
                None
            });

        (user_id, scopes)
    }
}
