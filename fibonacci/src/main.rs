use std::io;

fn fib(n: u32) -> u32 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fib (n-1) + fib(n-2);
    }
}

fn main() {
    println!("to quit the program type `exit` ");
    loop {
        println!("type a positive number");
        let mut int = String::new();
        io::stdin()
            .read_line(&mut int)
            .expect("Failed to read your input.");
            if int.trim() == "exit" {
                break;
            }
        let int: u32 = match int.trim()
            .parse() {
                Ok(int) => int,
                Err(_) => continue,
            };
        println!("Fibonacci ({int}) => {}", fib(int));
    }
}

