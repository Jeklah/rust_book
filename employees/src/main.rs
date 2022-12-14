// using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company.
// For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
// Bonus: Alphabetize the entries in the hash map by department.

use std::collections::HashMap;
use std::io;

fn main() -> ! {
    let mut company = HashMap::new();

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let mut input = input.split_whitespace();

        let command = input.next().unwrap();
        let name = input.next().unwrap();
        let department = input.next().unwrap();

        if command == "Add" {
            company.entry(department).or_insert(Vec::new()).push(name);
        } else if command == "List" {
            if department == "All" {
                for (dept, names) in &company {
                    println!("{}:", dept);

                    for name in names {
                        println!("\t{}", name);
                    }
                }
            } else {
                if let Some(names) = company.get(department) {
                    println!("{}:", department);

                    for name in names {
                        println!("\t{}", name);
                    }
                }
            }
        }
    }
}
