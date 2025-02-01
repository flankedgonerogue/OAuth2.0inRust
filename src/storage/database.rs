use crate::storage::{Client, User};
use log::error;
use tokio_postgres::Client as PgClient;

#[derive(Debug)]
pub struct Database {
    client: PgClient,
}

impl Database {
    pub fn new(client: PgClient) -> Self {
        Self { client }
    }

    pub async fn find_client(self: &Self, client_id: &u32) -> bool {
        let query = self
            .client
            .query(
                "SELECT id FROM clients WHERE id = $1::OID LIMIT 1;",
                &[client_id],
            )
            .await;

        if query.is_err() {
            error!("{}", query.err().unwrap());
            return false;
        }

        !query.unwrap().is_empty()
    }

    pub async fn get_client(self: &Self, client_id: &u32) -> Option<Client> {
        let query = self.client.query(
            "SELECT id, name, allowed_scopes, redirect_uris, secret FROM public.clients WHERE id = $1::OID LIMIT 1;", &[client_id]).await;

        if query.is_err() {
            error!("{}", query.err().unwrap());
            return None;
        }

        if let Some(row) = query.unwrap().into_iter().next() {
            return Option::from(Client {
                id: row.get(0),
                name: row.get(1),
                allowed_scopes: row.get(2),
                redirect_uris: row.get(3),
                secret: row.get(4),
            });
        }

        None
    }

    pub async fn get_user(self: &Self, email: &str, password: &str) -> Option<User> {
        let query = self.client.query(
            "SELECT id, email, password FROM users WHERE email = $1::VARCHAR AND password = $2::VARCHAR LIMIT 1", &[&email, &password],
        ).await;

        if query.is_err() {
            error!("{}", query.err().unwrap());
            return None;
        }

        if let Some(row) = query.unwrap().into_iter().next() {
            return Option::from(User {
                id: row.get(0),
                email: row.get(1),
                password: row.get(2),
            });
        }

        None
    }
}
