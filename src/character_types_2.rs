use std::mem::size_of;

fn main() {

    println!("Size of a char: {}", std::mem::size_of::<char>());
    println!("SIze of a char : {}", size_of::<char>());
    // .len == length() -> byte 수
    println!("sSize of string containing 'a' : {}", "a".len());
    println!("sSize of string containing '안녕' : {}", "안녕".len()); // byte 수 -> 결과 : 6
}