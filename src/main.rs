#[macro_use]
extern crate serde_derive;
extern crate indicatif;

use std::io;
use std::process;
use std::io::{Read, Write};
use serde_json::Value::String;
use crate::blockchain::Chain;

mod blockchain;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    get_input("input a mainer address: ", &mut miner_addr);
    get_input("Difficulty: ", &mut difficulty);
    let diff = difficulty.trim().parse::<u32>().expect("we needed");
    println!("generating genesis block! " );
    let mut chain = Chain::new(miner_addr.trim().to_string(), diff);

    loop {
        println!("Menu");
        println!("1) New Transaction");
        println!("2) Mine block");
        println!("3) Change Difficulty");
        println!("0) Exit");
        println!("Enter your choice: ");
        io::stdout().flush();
        choice.clear();
        io::stdin().read_line(&mut choice);
        println!("");

        match choice.trim().parse().unwrap() {
            0 =>
                {
                    println!("existing!");
                    process::exit(0);
                }
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                get_input("enter sender address: ", &mut sender);
                get_input("enter receiver address: ", &mut receiver);
                get_input("Enter amount: ", &mut amount);

                let res: bool = chain.new_transaction(sender.trim().to_string(),
                                                      receiver.trim().to_string(),
                                                      amount.trim().parse.unwrap);

                match res {
                    true => println!("Transaction added"),
                    false => println!("Transaction failed"),
                }
            }

            2 => {
                println!("Generating block");
                let res = chain.generate_new_block();
                match res {
                    true => println!("Block generated successfully"),
                    false => println!("Block generation failed"),
                }
            }

            3 => {
                let mut new_diff = String::new();
                get_input("enter new difficulty: ", &mut new_diff);
                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());

                match res {
                    true => println!("Difficulty update: {}", &new_diff),
                    false => println!("Update difficulty failed!")
                }
            }

            _ => println!("Invalid optional please retrt"),
        }
    }
}

fn get_input (ask_mesage: &str, s: &mut String) {
    println!("{}", ask_mesage);
    io::stdout().flush();
    io::stdin().read_line(s);
}