// using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company.
// For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
// Bonus: Alphabetize the entries in the hash map by department.

use std::collections::HashMap;
use std::io;
use std::iter::Iterator;

pub fn capitalise(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut input = String::new();

        println!("Enter command: ");
        io::stdin().read_line(&mut input).unwrap();

        let command: Vec<&str> = input.split(' ').collect();
        //let next_command: &str = command[0];
        let mut command: Vec<&str> = input.trim().split_whitespace().collect();
        let mut command = capitalise(command[0]);

        match command[0] {
            "Add" => {
                let employee = command[1];
                let department = command[2];

                company
                    .entry(department.to_string())
                    .or_insert(Vec::new())
                    .push(employee.to_string());
            }
            "List" => {
                if command.len() == 3 {
                    let department = command[2];
                    if let Some(employees) = company.get(department) {
                        for employee in employees {
                            println!("{}", employee);
                        }
                    } else {
                        println!("No employees found in department {}", department);
                    }
                } else {
                    for (department, employees) in &company {
                        println!("{}:", department);

                        for employee in employees {
                            println!("{}", employee);
                        }
                    }
                }
            }
            "Quit" => break,
            _ => println!("Invalid command"),
        }
    }
}
