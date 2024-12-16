use crate::storage::Client;
use crate::POSTGRES_CLIENT;

pub async fn check_client_id_in_db(client_id: u32) -> bool {
    let result = POSTGRES_CLIENT
        .get()
        .unwrap()
        .query(
            "SELECT client_id
                        FROM client_ids
                            WHERE client_id = $1::OID
                        LIMIT 1;
                      ",
            &[&client_id],
        )
        .await;

    result
        .unwrap_or_else(|err| {
            eprintln!("An error occurred in the query, returned an empty response to avoid panic!");
            eprintln!("{}", err);

            // Return an empty vector to avoid error since we are only comparing its length
            return vec![];
        })
        .len()
        > 0
}

pub async fn get_client_data_in_db(client_id: u32) -> Option<Client> {
    let result = POSTGRES_CLIENT
        .get()
        .unwrap()
        .query(
            "SELECT *

	FROM public.clients
	WHERE id = $1::OID
                        LIMIT 1;",
            &[&client_id],
        )
        .await;

    let result = result
        .unwrap_or_else(|err| {
            eprintln!("An error occurred in the query, returned an empty response to avoid panic!");
            eprintln!("{}", err);

            panic!("An error occurred in the query!");
        })
        .into_iter()
        .next()
        .unwrap();

    if result.is_empty() {
        None
    } else {
        Option::from(Client {
            id: result.get(0),
            name: result.get(1),
            allowed_scopes: result.get(2),
            redirect_uris: result.get(3),
        })
    }
}
