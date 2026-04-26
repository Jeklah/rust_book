fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let x = 5;
    let r = &x;

    println!("r: {}", r);

    let mut string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    string1 = String::from("long string is long");

    {
        let string3 = String::from("xyz");
        let result2 = longest(string1.as_str(), string3.as_str());
        println!("The longest string is {result2}");
    }
}
