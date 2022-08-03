use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let v = vec![1, 2, 3];

    // v[99];

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("problem creating the file {:?}", e),
            },
            other_error => {
                panic!("problem opening the file {:?}", other_error)
            }
        },
    };
    
    // same logic using closures, here |error|. | name | is the notation for closures
    let f2 = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("problem creating the file: {:?}", error);
            })
        } else {
            panic!("problem opening the file: {:?}",error);
        }
    });

    let f3 = File::open("hello.txt")?;
    Ok(())
}
// propagating errors to the caller
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// is the same as above, using the ? operator to propagate errors
// to calling code
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// and again
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// also can do this but no error handling then
fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
