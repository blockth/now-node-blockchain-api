#[macro_use]
extern crate serde_derive; // we are gonna use serde for deserialization from serde crate

// include all the other rust modules
mod blockchain_address;
mod blockchain_info;
mod blockchain_status;
mod blockchain_transaction;

use dotenv;
use std::{io, thread, time};

use crate::blockchain_info::blockchain_transaction_details_request;

fn get_blockchain_info(address: String) {
    let blockchain_status = blockchain_info::blockchain_status_request();

    println!(
        "Querying {}, -chain {} \n",
        &blockchain_status.blockbook.coin, &blockchain_status.backend.chain
    );
    let sleep_time = time::Duration::from_millis(2500);
    thread::sleep(sleep_time);
    let add = &address;
    let blockchain_address = blockchain_info::blockchain_address_request(add.to_string());
    // println!(
    //     "Queried Address: {}, Address Response: {}, transactions: {:?}",
    //     add, blockchain_address.address, blockchain_address.txids
    // );
    println!(
        "Queried Address: {}, Address Response: {}, transactions length {}",
        add,
        blockchain_address.address,
        blockchain_address.txids.len()
    );

    println!("To see the details of some of transactions select 1 \n ");
    println!("To estimate the balance of the account select 2\n ");
    thread::sleep(sleep_time);
    let mut command = String::new();

    std::io::stdin().read_line(&mut command);

    if command.trim().eq("1") {
        let mut input = String::new();
        println!("Enter partition size:");
        std::io::stdin().read_line(&mut input);
        let partition = input.trim().parse().unwrap();
        let mut i = 0;
        while i < partition {
            thread::sleep(sleep_time);
            let transaction_details =
                blockchain_transaction_details_request(blockchain_address.txids[i].to_string());
            println!(
                "Transaction ID: {:#}\n trx Vin: {:?} \n trx Vout: {:?} \n block height: {}, block time:{}",
                blockchain_address.txids[i], transaction_details.vin, transaction_details.vout, transaction_details.block_height, transaction_details.block_time
            );
            i += 1;
        }
    } else if command.trim().eq("2") {
        let mut balance = 0;
        thread::sleep(sleep_time);
        for trx in blockchain_address.txids {
            thread::sleep(sleep_time);
            let mut subtotal_vin = 0;
            let mut subtotal_vout = 0;

            let trx = blockchain_info::blockchain_transaction_details_request(trx.to_string());
            for t in trx.vin {
                if t.addresses.contains(add) {
                    subtotal_vin += t.value.parse::<i32>().unwrap();
                }
            }

            for t in trx.vout {
                if t.addresses.contains(add) {
                    subtotal_vout += t.value.parse::<i32>().unwrap();
                }
            }
            balance += &subtotal_vin - &subtotal_vout;

            println!("------------------------------------");
            println!("Trx ID : {}", trx.txid);
            println!("Satoshis In: {}", subtotal_vin);
            println!("Satoshis In: {}", subtotal_vout);
            println!("Estimated balance of {} is: {}", add, balance.to_string());

            println!("------------------------------------");
        }
    } else {
        println!("Gracefully exiting...");
        thread::sleep(sleep_time);
    }
}

#[allow(unused)]
fn main() {
    get_blockchain_info(dotenv::var("WALLET").expect("WALLET address is not found in .env file"));
}
