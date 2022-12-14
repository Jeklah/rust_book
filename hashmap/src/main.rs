use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    let initial_scores = vec![10, 50];

    // let mut scores: HashMap<_, _> =
    //     teams.into_iter().zip(initial_scores.into_iter()).collect();
    let field_name = String::from("Favorite colour");
    let field_value = String::from("Blue");

    // When you want to change the data in a hash map, you have to decide how to
    // handle the case when the key already has a value. You could replace the old
    // value with the new value, completely disregarding the old value. You could
    // keep the old value and ignore the new value. Or you could combine the two
    // values into one new value. We’re going to look at how to do the last of
    // these options, which is sometimes called combining, accumulating, or
    // reducing.

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
