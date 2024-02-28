mod to_do;
mod state;
mod processes;

use state::{read_file};
use serde_json::value::Value;
use serde_json::{Map};

use to_do::ItemTypes;
use to_do::to_do_factory;

use std::env;
use crate::processes::process_input;

fn check_binary_environment() {
    let args: Vec<String> = env::args().collect();
    let path: &str = &args[0];
    if path.contains("/debug/") {
        println!("The development app is running");
    } else if path.contains("/release/") {
        println!("The production server is running");
    } else {
        panic!("The setting is neither debug nor release");
    }
}

fn main() {
    check_binary_environment();

    let to_do: ItemTypes = to_do_factory("pending", "make").unwrap();
    match to_do {
        ItemTypes::Pending(p) => {
            println!("Pending item: {}", p.super_struct.title);
        }
        ItemTypes::Done(d) => {
            println!("Done item: {}", d.super_struct.title);
        }
    }

    let args: Vec<String> = env::args().collect();
    let command: &String = &args[1];
    let title: &String = &args[2];
    let state: Map<String, Value> = read_file("./state.json");
    let status: String;
    match &state.get(*&title) {
        Some(result) => {
            status = result.to_string().replace('\"', "");
        }
        None=> {
            status = "pending".parse().unwrap();
        }
    }
    let item = to_do_factory(&status, title).expect(&status);
    process_input(item, command.to_string(), &state);
}
