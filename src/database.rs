use crate::storage::{Client, User};
use crate::{POSTGRES_CLIENT};

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
            eprintln!("{}", err);
            panic!("An error occurred in the GET_CLIENT_DATA_IN_DB query!");
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

pub async fn find_user_in_db(email: &String, password: &String) -> Option<User> {
    let result = POSTGRES_CLIENT.get().unwrap().query(
        "SELECT id, email, password FROM users WHERE email = $1::VARCHAR AND password = $2::VARCHAR LIMIT 1",&[email, password]
    ).await;
    
    let result =  result.unwrap_or_else(|err| {
        eprintln!("{}", err);
        panic!("An error occurred in the FIND_USER_IN_DB query!");
    }).into_iter().next().unwrap();
    
    if result.is_empty() {
        None
    } else {
        Option::from(User {
            id: result.get(0),
            email: result.get(1),
            password: result.get(2),
        })
    }
}
