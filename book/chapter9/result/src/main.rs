// enum Result<T, E> { // T and E are both generic types that represent the types of the returned values in both cases
//     Ok(T),
//     Err(E),
// }

use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}"),
            }
        },
    };

    let greeting_file = File::open("hello.txt").unwrap(); // will return the file or call panic!
    // expect lets you choose the error message which is then passed to panic! - still returns the file if Ok
    let greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project")
}

// "propagating errors" sends the result up to the calling code so it can decide what to do with it
// will return an Ok containing the username, or an Err with an io:Error
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// the ? operator gives Ok and the fn continues or returns an Err out of the function, propagating the error
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
// just a shorter version by chaining the method calls with the ?s
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// ? can be used with functions that return an Option
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
