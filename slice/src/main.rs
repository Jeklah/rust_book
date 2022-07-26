// write a function that takes a string of words separated by spaces and
// returns the first word it finds in that string. if no spaces, return
// entire string.
//
// fn first_word(s: &String) -> ?
//
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
// 
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {          failed implementation.
//             return i;
//         }
//     }
// 
//     s.len()
// }

// correct implementation
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


fn main() {
    let mut s = String::from("hello world");
    
    let word = first_word(&s); // word will get the value 5
    let hello = &s[0..5];
    let world = &s[6..11]; // end_index is one more than ending position
    
    // these are equal
    let slice = &s[0..2];
    let slice = &s[..2];
    // similarly you can do the same with the end_index
    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];
    // you can also drop both for the whole string, so these are equal
    let slice = &s[0..len];
    let slice = &s[..];

    //s.clear(); // this empties the String, making it equal to ""
    println!("{word}");
    println!("{hello} {world}");
    // word still has the value 5 here, but theres no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid

    let my_string = String::from("hello world!");

    // 'first_word' works on slices of 'String's, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // 'first_word' also works on references to 'String's, which are equivalent
    // to whole slicess of 'String's
    let word = first_word(&my_string);

    let my_string_literal = "hello world!";

    // 'first_word' works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
