// using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company.
// For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a
// department or all people in the company by department, sorted alphabetically.
// Bonus: Alphabetize the entries in the hash map by department.

use std::collections::HashMap;
use std::io;
use std::iter::Iterator;

pub fn capitalise(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first_char) => first_char.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut input = String::new();
        println!("Enter command: ");
        io::stdin().read_line(&mut input).unwrap();
        let command_string: Vec<&str> = input.trim().split_whitespace().collect();
        let first_command: &str = &capitalise(command_string[0]);

        match first_command {
            "Add" => {
                let employee = command_string[1];
                let department = command_string[3];
                company
                    .entry(department.to_string())
                    .or_insert(Vec::new())
                    .push(employee.to_string());
            }
            "List" => {
                if command_string.len() == 2 {
                    let department = command_string[1];
                    let print_department = capitalise(department);
                    if let Some(employees) = company.get(department) {
                        println!("Employees in {}:", print_department);
                        for employee in employees {
                            let print_employee = capitalise(employee);
                            println!("{}", print_employee);
                        }
                    } else {
                        println!("No employees found in department {}", department);
                    }
                } else {
                    for (department, employees) in &company {
                        let print_department = capitalise(department);
                        println!("Department: {}:", print_department);

                        for employee in employees {
                            let print_employee = capitalise(employee);
                            println!("{}", print_employee);
                        }
                    }
                }
            }
            "Quit" => break,
            _ => println!("Invalid command. Please use 'Add', 'List', or 'Quit'."),
        }
    }
}
