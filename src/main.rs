#![allow(non_snake_case)] //For allowing non-snake Case naming convention e.g. IsDone
use std::fs;
use std::fs::File;
// use std::fs::OpenOptions;
use std::io::{self, Read}; //For standard input
use std::io::{BufRead, Write}; //For standard output
use std::path::Path; //For path of file
use std::thread; //For introducing delays
use std::time::Duration; //For adding time delays //For split strings // For appending to file

struct Inventory {
    name: String,
    item_id: String,
    category: String,
    item_count: String,
    allocated_to: Vec<String>,
}

fn main() {
    let inventory: &mut Vec<Inventory> = &mut Vec::new();
    update_inventory(inventory);
    // update_file(inventory);
    // menu(inventory);
    // delete_item(inventory);
    view_member(inventory);
}

fn menu(inv: &mut Vec<Inventory>) -> u32 {
    let option: &mut String = &mut String::new();

    println!("---- Faculty Inventory System ----");
    println!("Select an option from one of the following:");
    while option.to_string().trim() != "0" {
        String::clear(option);
        println!("1. Add inventory items");
        println!("2. View all inventory items");
        println!("3. Search inventory items");
        println!("4. Edit inventory items");
        println!("5. Delete inventory items");
        println!("6. Assign inventory items");
        println!("7. Retrieve inventory items");
        println!("8. View list of members with specific inventory items");
        println!("0. Exit\n");

        print!("Enter option number: ");
        io::stdout().flush().expect("Could not flush stdout");
        io::stdin()
            .read_line(option)
            .expect("Could not get user input");

        // println!("The value of option is: {}", option);

        println!("You entered option: {}", option);
        if option.trim() == "1" {
            add_item(inv);
        } else if option.trim() == "2" {
            view_item(inv);
        } else if option.trim() == "3" {
            search_item(inv);
        } else if option.trim() == "4" {
        } else if option.trim() == "5" {
            delete_item(inv);
        } else if option.to_string().trim() == "6" {
            assign_item(inv);
        } else if option.trim() == "7" {
        } else if option.trim() == "8" {
        } else if option.trim() == "0" {
            println!("\n[INFO]: Good Bye !\n");
            thread::sleep(Duration::from_secs(2));
        } else {
            println!("\n[Error]: You did not enter a correct option !\n");
            thread::sleep(Duration::from_secs(2));
        }
    }
    return 0;
}

fn update_inventory(inv: &mut Vec<Inventory>) {
    inv.clear();
    let mut isnewfile: bool = false;
    let mut _line_count = 0;
    // let inv_vector: Vec<Inventory> = vec![]; // Created a vector of type inventory for working with file handling

    // inv_vector.push(Inventory { name: String::from("Printer"), item_id: String::from("23"), category: String::from("Machines"), item_count: 1, allocated_to: String::from("Teacher1") });

    let path = Path::new("inventory_item_data.txt");

    let f = match File::open(path) {
        Ok(file) => {
            println!("\n\n [INFO]: Database file found!\n");
            file
        } //Means file found
        Err(reason_read) => match File::create(path) {
            //Means file not found so creating a new one
            Ok(file) => {
                isnewfile = true;
                println!("\n\n [INFO]: No file found so creating a new file!\n");
                file
            }
            Err(reason_write) => panic!(
                "Error creating or reading file due to: {} and {}",
                reason_read, reason_write
            ),
        },
    };
    if !isnewfile {
        let all_lines = io::BufReader::new(f); //For reading lines of file

        for line in all_lines.lines() {
            match line {
                Ok(line) => {
                    let line_slice: Vec<&str> = line.split(",").collect::<Vec<&str>>(); //collect() is used for converting Split type to string

                    let mut allocated_to_slice: Vec<String> = Vec::new();
                    for k in line_slice[4].split("|").collect::<Vec<&str>>() {
                        allocated_to_slice.push(k.to_string());
                    }

                    inv.push(Inventory {
                        name: line_slice[0].to_string(),
                        item_id: line_slice[1].to_string(),
                        category: line_slice[2].to_string(),
                        item_count: line_slice[3].to_string(),
                        allocated_to: allocated_to_slice,
                    })
                }
                Err(reason) => panic!("Could not get line from file due to reason: {}", reason),
            }
        }
    }
}

fn update_file(inv: &mut Vec<Inventory>) {
    fs::write("inventory_item_data.txt", "").expect("Error writing the file"); // Emptied the file
    let mut f = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("inventory_item_data.txt")
        .unwrap();

    for i in inv {
        write!(f, "{}", i.name.to_string().trim()).expect("Error appending the file");
        write!(f, ",").expect("Error appending the file");
        write!(f, "{}", i.item_id.to_string().trim()).expect("Error appending the file");
        write!(f, ",").expect("Error appending the file");
        write!(f, "{}", i.category.to_string().trim()).expect("Error appending the file");
        write!(f, ",").expect("Error appending the file");
        write!(f, "{}", i.item_count.to_string().trim()).expect("Error appending the file");
        write!(f, ",").expect("Error appending the file");
        let size = i.allocated_to.len();
        for j in 0..size {
            write!(f, "{}", i.allocated_to[j].to_string().trim())
                .expect("Error appending the file");
            if j != size - 1 {
                write!(f, "|").expect("Error appending the file");
            }
        }
        write!(f, "\n").expect("Error appending the file");
    }
}

fn add_item(inv: &mut Vec<Inventory>) {
    let item_name: &mut String = &mut String::new();
    let item_category: &mut String = &mut String::new();
    let item_count: &mut String = &mut String::new();
    let item_id: &mut String = &mut String::new();

    print!("\nEnter name of item: ");
    io::stdout().flush().expect("Could not flush stdout");
    io::stdin()
        .read_line(item_name)
        .expect("Could not read user input");

    print!("\nEnter item category: ");
    io::stdout().flush().expect("Could not flush stdout");
    io::stdin()
        .read_line(item_category)
        .expect("Could not read user input");

    print!("\nEnter item count: ");
    io::stdout().flush().expect("Could not flush stdout");
    io::stdin()
        .read_line(item_count)
        .expect("Could not read user input");

    print!("\nEnter item ID: ");
    io::stdout().flush().expect("Could not flush stdout");
    io::stdin()
        .read_line(item_id)
        .expect("Could not read user input");

    inv.push(Inventory {
        name: item_name.to_string(),
        item_id: item_id.to_string(),
        category: item_category.to_string(),
        item_count: item_count.to_string(),
        allocated_to: Vec::new(),
    });
    update_file(inv);
}

fn view_item(inv: &mut Vec<Inventory>) {
    for i in inv.iter() {
        print!("{}\t", i.name.to_string());
        print!("{}\t", i.item_id);
        print!("{}\t", i.category);
        print!("{}\t", i.item_count);
        for j in i.allocated_to.iter() {
            print!("{}\t", j);
        }
        println!();
    }
}

fn search_item(inv: &mut Vec<Inventory>) {
    let mut is_found = false;
    let item_name: &mut String = &mut String::new();

    print!("Enter the name of the item to search: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(item_name)
        .expect("Could not read user input");

    for i in inv.iter() {
        if item_name.to_string().trim() == i.name.to_string().trim() {
            print!("{}\t", i.name.to_string());
            print!("{}\t", i.item_id);
            print!("{}\t", i.category);
            print!("{}\t", i.item_count);
            for j in i.allocated_to.iter() {
                print!("{}\t", j);
            }
            println!();
            is_found = true;
            break;
        }
    }
    if !is_found {
        println!("Item not found !");
    }
}

fn delete_item(inv: &mut Vec<Inventory>) {
    view_item(inv);

    let item_name: &mut String = &mut String::new();

    print!("Enter the name of item you want to delete: ");
    io::stdout().flush().expect("Error flushing stdout");

    io::stdin()
        .read_line(item_name)
        .expect("[Error]: Could not read user input !");

    for i in 0..inv.len() {
        if item_name.to_string().trim() == inv[i].name.to_string().trim() {
            inv.remove(i);
            break;
        }
    }
    update_file(inv);
}

fn assign_item(inv: &mut Vec<Inventory>) {
    let mut is_found = false;
    view_item(inv);

    let item_name: &mut String = &mut String::new();
    let allocate_to: &mut String = &mut String::new();
    let item_count: u32;

    print!("Enter the name of item you want to assign: ");
    io::stdout()
        .flush()
        .expect("[Error]: Could not flush stdout");
    io::stdin()
        .read_line(item_name)
        .expect("[Error]: Could not read item_name from user !");

    print!("Enter the name of person you want to assign the item to: ");
    io::stdout()
        .flush()
        .expect("[Error]: Could not flush stdout");
    io::stdin()
        .read_line(allocate_to)
        .expect("[Error]: Could not read allocate_to from user !");

    for i in 0..inv.len() {
        if inv[i].name.to_string().trim() == item_name.to_string().trim() {
            println!("\n [INFO]: Found Item !\n");
            item_count = inv[i].item_count.parse::<u32>().unwrap() - 1;
            inv[i].item_count = item_count.to_string();
            println!("Length before: {}", inv[i].allocated_to.len());
            inv[i].allocated_to.push(allocate_to.trim().to_string());
            is_found = true;
            println!("Length after: {}", inv[i].allocated_to.len());
            break;
        }
    }
    if is_found == false {
        println!("[INFO]: No item found!");
    }
    update_file(inv);
}

fn view_member(inv: &mut Vec<Inventory>) {
    let item_name: &mut String = &mut String::new();

    print!("Enter the item name to view its members: ");
    io::stdout()
        .flush()
        .expect("[ERROR]: Failed to flush stdout");

    io::stdin()
        .read_line(item_name)
        .expect("[ERROR]: Error reading user input for item_name");

    for i in inv.iter() {
        if i.name.to_string().trim() == item_name.to_string().trim() {
            println!(
                "\nFollowing are the faculty members who borrowed {}\n",
                item_name
            );
            for j in i.allocated_to.iter() {
                println!("{}", j);
            }
        }
    }
}
