// pig latin
// convert strings to pig latin
// "hello" => "ellohay"
use std::io;

fn piglatin(s: &str)  {
    let mut result = String::new();
    let mut chars = s.chars();
    let first_char = chars.next().unwrap();
    if first_char.is_alphabetic() {
        result.push(first_char);
        result.push_str("ay");
    }
    for c in chars {
        if c.is_alphabetic() {
            result.push(c);
        }
    }
    println!("{}", result);
}

fn main() {
    println!("Please enter a string.");
    // get user input
    
    // let s = String::new();
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("failed to read input");
    piglatin(&s);
}
