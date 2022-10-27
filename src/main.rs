#[macro_use]
extern crate serde_derive; // we are gonna use serde for deserialization from serde crate

// include all the other rust modules
mod blockchain_address;
mod blockchain_info;
mod blockchain_status;
mod blockchain_transaction;

#[allow(unused)]
fn main() {
    let blockchain_status = blockchain_info::blockchain_status_request();

    println!(
        "Querying {}, -chain {} \n",
        &blockchain_status.blockbook.coin, &blockchain_status.backend.chain
    )
}
