use std::io::Write; //For standard output
use std::time::Duration;
use std::{io, option}; //For standard input
use std::{thread, time}; //For introducing delays
fn main() {
    menu();
}

fn menu() {
    let mut option: String = String::new();

    println!("---- Faculty Inventory System ----");
    println!("Select an option from one of the following:");

    println!("1. Add inventory items");
    println!("2. View all inventory items");
    println!("3. Search inventory items");
    println!("4. Edit inventory items");
    println!("5. Delete inventory items");
    println!("6. Assign inventory items");
    println!("7. Retrieve inventory items");
    println!("8. View list of members with specific inventory items\n");

    print!("Enter option number: ");
    io::stdout().flush();
    io::stdin().read_line(&mut option);
    if option == "1" {
        menu();
    } else if option == "2" {
        menu();
    } else if option == "3" {
        menu();
    } else if option == "4" {
        menu();
    } else if option == "5" {
        menu();
    } else if option == "6" {
        menu();
    } else if option == "7" {
        menu();
    } else if option == "8" {
        menu();
    } else {
        println!("\n[Error]: You did not entered a correct option !\n");
        thread::sleep(Duration::from_secs(2));
        menu();
    }
}
