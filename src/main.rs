#[macro_use]
extern crate lazy_static;

use std::io;
use std::collections::HashMap;
use regex::Regex;

lazy_static! {
    static ref ADD_REGEX: Regex = Regex::new("Add (.*) to (.*)").unwrap();
}

fn handle_one (department_employees:&mut HashMap<String, Vec<String>>) {
    println!("  Who do you want to add to which department? (e.g. Add Sally to Engineering)");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();
    
    let cap = match ADD_REGEX.captures(&input) {
        Some(cap) => cap,
        None => { 
            println!("Invalid input");
            return;
        },
    };

    department_employees
        .entry(cap[2].to_string().to_lowercase())
        .or_insert_with(|| Vec::new())
        .push(cap[1].to_string());
}

fn handle_two (department_employees: &HashMap<String, Vec<String>>) {
    println!("  Which department? (e.g. Engineering, All)");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim().to_lowercase();
    if input != "all" {
        println!("fetching {}", &input);
        match department_employees.get(&input) {
            Some(employees) => {
                for employee in employees {
                    println!("*** {}", &employee);
                }
            },
            None => {
                println!("There is no employee in {}", input);
            },
        }
        return;
    }

    for employees_in_department in department_employees.values() {
        for employee in employees_in_department {
            println!("*** {}", employee);
        }
    }
}

fn main() {
    let mut department_employees: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("");
        let mut input = String::new();
        println!("Welcome to Department Employees management tool!\nPlease input your choice:");
        println!("  1. Add employee to Department");
        println!("  2. List all employees in a department or the whole company (e.g. Engineering, All)");

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if choice == 1 {
            handle_one(&mut department_employees);
            continue;
        }

        if choice == 2 {
            handle_two(&department_employees);
            continue;
        }

        println!("Invalid choice!");
    }
}
