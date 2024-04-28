use mongodb::{options::ClientOptions, Client, error::Error};
use std::env;

pub async fn form_connection() -> Result<Client, Error> {
    let mongodb_uri =
        env::var("MONGODB_URI").expect("Error! MONGODB_URI is not available in the .env file.");

    let client_options = ClientOptions::parse(mongodb_uri).await?;

    return Client::with_options(client_options);
}
