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

// lets get the address details
// HOST_ROOT=https://eth-blockbook.nownodes.io/api/ + we need to add  v2/address/0xD3e855Fa9564f747571C4324695c20AAC726B1Ad

pub fn blockchain_address_request(address: String) -> BlockchainAddres {
    let query = &[HOST_ROOT, "v2/address/", &address].join("");
    // println!("Query is {}", query);
    let response = send_request(&query); //delimeter none to join all
                                         // println!("{}", response);
    serde_json::from_str(&response).expect("Failed to parse JSON")
}
//https://eth-blockbook.nownodes.io/api/ + v2/tx/0x09bfdb7a8d284fefbd69ef2e35e173774223650dd4af7aa5a5d0bd291a916c17
pub fn blockchain_transaction_details_request(address: String) -> BlockchainTransaction {
    let query: &String = &[HOST_ROOT, "v2/tx/", &address].join("");
    println!("Query is {}", query);
    let response = send_request(&query);
    serde_json::from_str(&response).expect("Failed to parse JSON")
}
