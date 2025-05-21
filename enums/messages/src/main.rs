enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
            Message::Write(text) => println!("Write: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!("Change color to r: {}, g: {}, b: {}", r, g, b)
            }
        }
    }
}

fn main() {
    let m = Message::Move { x: 10, y: 20 };
    m.call();

    let m = Message::ChangeColor(255, 0, 0);
    m.call();

    let m = Message::Quit;
    m.call();
}
