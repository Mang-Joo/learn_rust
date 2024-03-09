use std::error::Error;
use std::{fs, io};
use std::fs::File;
use std::io::{ErrorKind, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file_result = File::open("hello.txt");

    let mut greeting_file = greeting_file_result.unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file : {:?}", error);
        }
    });


    let mut str = String::new();
    greeting_file.read_to_string(&mut str).expect("Can't read string");

    println!("str => {}", str);


    let result = read_username_from_file();

    let result_v2 = read_username_from_file_v2();

    let result_v3 = read_username_from_file_v3();


    match result {
        Ok(value) => { println!("result v1 is {}", value) }
        Err(e) => panic!("Error {}", e),
    }

    println!("result v2 is {}", result_v2?);
    println!("result v3 is {}", result_v3?);

    Ok(())
}

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

fn read_username_from_file_v2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();

    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_v3() -> Result<String, io::Error> {
    fs::read_to_string("none.txt")
}