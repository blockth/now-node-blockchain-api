use dotenv; //read .env file
use reqwest; // http requests
use serde_json::Result; // for serialization
use tokio; // for asynchronous threads

use crate::blockchain_address::BlockchainAddres;
use crate::blockchain_status::BlockchainStatus; // pull another
use crate::blockchain_transaction::BlockchainTransaction;

const HOST_ROOT: &str = "https://eth-blockbook.nownodes.io/api/";

#[tokio::main]
pub async fn send_request(url: &str) -> String {
    let client = reqwest::Client::new();
    client
        .get(url)
        .header(
            "api-key",
            dotenv::var("API_KEY") // added api key.it will return an Option,
                .expect("Could not find API Key"),
        ) //  if its None => expect err
        .send()
        .await //tokio is going to be conducting this
        .expect("Failed to get response") // if we can not get the response
        .text() //format it into string representation of json
        .await
        .expect("Failed to convert payload")
}

pub fn blockchain_status_request() -> BlockchainStatus {
    let response = send_request(&HOST_ROOT);
    // println!("{}", response);

    //it returns the Option so handle None and panic error
    serde_json::from_str(&response).expect("Failed to parse JSON")
}
