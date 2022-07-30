// pig latin
// convert strings to pig latin
// "hello" => "ellohay"

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
    // get user input
    println!("Please enter a string."); 
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("input");
    let words = s.trim().split(' ');
    for word in words {
        piglatin(word)
    }
}
