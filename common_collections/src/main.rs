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
}
