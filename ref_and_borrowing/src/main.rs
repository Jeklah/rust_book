fn main() {
    let mut s1 =  String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);
    change(&mut s1);
    some_func();
}

fn calculate_length(s1: &String) -> usize {
    s1.len()
} // Here s1 goes out of scope, but because it does not hae ownership of what
  // it refers to, it s not dropped. 

fn change(some_string: &mut String) {
    some_string.push_str(" world");
    println!("{some_string}");
}

fn some_func() {
    // multiple references to same thing = prob
    let mut s = String::from("hello");
    
    // let r1 = &mut s;  cant borrow twice
    // let r2 = &mut s;
    
    // println!("{}, {}", r1, r2);
    {
       let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    
    let r2 = &mut s;
    
    // let mut s = String::from("hello");
    //
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    //
    // println!("{}, {}, and {}", r1, r2, r3);
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}
