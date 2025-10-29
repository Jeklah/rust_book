fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    v.push(6);
    v.push(7);
    v.push(8);
    v.push(9);

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // let does_not_exist = &v[100];
    // let doesnt_exist = v.get(100);

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // let _does_not_exist = &v[100];
    // let _does_not_exist = v.get(100);

    let v2 = vec![100, 32, 57];
    for i in &v2 {
        println!("{i}");
    }

    let mut v3 = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    let mut testv: Vec<i32> = vec![1, 2, 3];
    let mut testv2: Vec<&mut i32> = Vec::new();
    for i in &mut testv {
        testv2.push(i);
    }
    *testv2[0] = 5;

    let a = *testv2[0];
    let b = testv[0];
    println!("{a}, {b}");

    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    // let hello = String::from("السلام عليكم");
    // let hello = String::from("Dobrý den");
    // let hello = String::from("Hello");
    // let hello = String::from("שלום");
    // let hello = String::from("नमस्ते");
    // let hello = String::from("こんにちは");
    // let hello = String::from("안녕하세요");
    // let hello = String::from("你好");
    // let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    // let hello = String::from("Hola");

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved, while s2 is being borrowed/referenced
    println!("{}", s3);

    let str1 = String::from("tic");
    let str2 = String::from("tac");
    let str3 = String::from("toe");
    let str = str1 + "-" + &str2 + "-" + &str3; // the maximum number of times a heap allocation
    // could occur in this bit of code is 7. one for
    // each call to String::from, and then 4 more for
    // each concatenation.
    println!("{}", str);

    let strg1 = String::from("tic");
    let strg2 = String::from("tac");
    let strg3 = String::from("toe");
    let strg = format!("{strg1}-{strg2}-{strg3}"); // format! returns a new String with the
    // contents without taking ownership of any of
    // the parameters
    println!("{}", strg);

    let s = &hello[0..4];
    println!("{}", s);

    // Iterating over strings
    for c in "Здравствуйте".chars() {
        println!("{}", c);
    }

    for c in hello.chars() {
        println!("{}", c);
    }

    for c in hello.bytes() {
        println!("{}", c);
    }
}
