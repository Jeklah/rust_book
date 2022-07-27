fn main() {
    // let ref_to_nothing = dangle(); won't compile
    let ref_to_nothing = no_dangle(); // will compile
}

// fn dangle() -> &String { // dangle returns a reference to a String
//    let s = String::from("hello"); // s is a new String

//    &s // we return a reference to the String, s
//} // Here, s goes out of scope(!), and is dropped. Its memory goes away.
//  // Danger!


fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
