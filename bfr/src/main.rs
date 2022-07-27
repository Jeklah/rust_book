use std::{env, fs};
mod interpreter;

fn main() {
    let file_name = env::args().nth(1).expect("Please enter the path to the source: ");
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading this file.");

    let mut machine = interpreter::Machine::default();
    machine.parse_instructions(contents);
    machine.execute_instructions();
}
