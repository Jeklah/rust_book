fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v2 = vec![100, 32, 57];
    for i in &v2 {
        println!("{}", i);
    }

    let mut v3 = vec![100, 32, 57];
    for i in &mut v3 {
        *i += 50;
    }

    // An example of vectors holding multiple types thanks to using
    // enums.
    //

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
}
