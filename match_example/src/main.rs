enum Coin {
    Penny
    ,Nickel
    ,Dime
    ,Quarter(UsState)
}

#[derive(Debug)] //so we can inspect the state
enum UsState {
    Alabama
    ,Alaska
    ,Colorado
    ,Texas
    ,NewVegas
    ,Washington
    ,California
}


fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!!");
            1
        } 
        ,Coin::Nickel => 5
        ,Coin::Dime => 10
        ,Coin::Quarter(state) => {
            println!("state quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    let coin = Coin::Quarter(UsState::Alaska);
    let coin2 = Coin::Quarter(UsState::Alabama);

    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("state quarter from {:?}", state),
        _ => count += 1,
    }

    // or we can use if let with else 
    if let Coin::Quarter(state) = coin2 {
        println!("state quarter from {:?}", state);
    } else {
        count += 1;
    }
}
