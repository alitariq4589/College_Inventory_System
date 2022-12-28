use std::fs;
use std::fs::File;
// use std::fs::OpenOptions;
use std::io; //For standard input
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
    update_file(inventory);
    // menu();
}

fn menu() {
    let mut option = String::new();

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
    io::stdout().flush().expect("Could not flush stdout");
    io::stdin().read_line(&mut option).expect("Could not get user input");

    // println!("The value of option is: {}", option);

    if option.trim() == "a" {
        menu();
    } else if option.trim() == "2" {
        menu();
    } else if option.trim() == "3" {
        menu();
    } else if option.trim() == "4" {
        menu();
    } else if option.trim() == "5" {
        menu();
    } else if option.trim() == "6" {
        menu();
    } else if option.trim() == "7" {
        menu();
    } else if option.trim() == "8" {
        menu();
    } else {
        println!("\n[Error]: You did not enter a correct option !\n");
        thread::sleep(Duration::from_secs(2));
        menu();
    }
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
        write!(f, "{}", i.name.to_string()).expect("Error appending the file");
        write!(f, ",").expect("Error appending the file");
        write!(f, "{}", i.item_id.to_string()).expect("Error appending the file");
        write!(f, ",").expect("Error appending the file");
        write!(f, "{}", i.category.to_string()).expect("Error appending the file");
        write!(f, ",").expect("Error appending the file");
        write!(f, "{}", i.item_count.to_string()).expect("Error appending the file");
        write!(f, ",").expect("Error appending the file");
        let size = i.allocated_to.len();
        for j in 0..size {
            write!(f, "{}", i.allocated_to[j].to_string()).expect("Error appending the file");
            if j != size-1{
            write!(f, "|").expect("Error appending the file");
            }
        }
        // write!(f, "wat\u00000008wat").expect("Error appending the file");
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
    io::stdin().read_line(item_name).expect("Could not read user input");

    print!("\nEnter item category: ");
    io::stdout().flush().expect("Could not flush stdout");
    io::stdin().read_line(item_category).expect("Could not read user input");

    print!("\nEnter item count: ");
    io::stdout().flush().expect("Could not flush stdout");
    io::stdin().read_line(item_count).expect("Could not read user input");

    print!("\nEnter item ID: ");
    io::stdout().flush().expect("Could not flush stdout");
    io::stdin().read_line(item_id).expect("Could not read user input");

    inv.push(Inventory {
        name: item_name.to_string(),
        item_id: item_id.to_string(),
        category: item_category.to_string(),
        item_count: item_count.to_string(),
        allocated_to: Vec::new(),
    });
    update_file(inv);
}
