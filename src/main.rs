mod to_do;
use to_do::ItemTypes;
use to_do::to_do_factory;
use to_do::structs::traits::create::Create;

use rand::prelude::*;
use std::env;

/// Generates a random float number between 0 and 10.
///
/// # Arguments
/// * `generator` (&mut ThreadRng): The random number generator.
///
/// # Returns
/// A random number between 0 and 10 as f64.
fn generate_float(generator: &mut ThreadRng) -> f64 {
    let r: f64 = generator.gen();
    r * 10.0
}

/// Trait defining basic user behavior.
trait IsUser {
    /// Checks if the struct instance is a user.
    ///
    /// # Returns
    /// Always returns true for user instances.
    fn is_user() -> bool {
        true
    }
}

/// Represents a user with a name and age.
///
/// # Attributes
/// * `name` (String): The name of the user.
/// * `age` (i8): The age of the user.
struct User {
    name: String,
    age: i8,
}

/// Checks the environment the binary is running in.
///
/// It identifies if the running environment is development, production, or neither.
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

fn generate_number() {
    // Generate and print a random number
    let mut rng: ThreadRng = rand::thread_rng();
    let r: f64 = generate_float(&mut rng);
    println!("Random number: {}", r);

    // Print command-line arguments
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);
}

fn main() {
    check_binary_environment();
    println!("Hello, world!");
    generate_number();

    let to_do: ItemTypes = to_do_factory("pending", "make").unwrap();
    match to_do {
        ItemTypes::Pending(p) => {
            println!("Pending item: {}", p.super_struct.title);
        }
        ItemTypes::Done(d) => {
            println!("Done item: {}", d.super_struct.title);
        }
    }
    let washing_to_do: ItemTypes = to_do_factory("pending", "washing").unwrap();
    match washing_to_do {
        ItemTypes::Pending(item) => item.create(
            &item.super_struct.title),
        ItemTypes::Done(item) => println!(
            "it's a done item with the title: {}",
            item.super_struct.title)
    }
}
