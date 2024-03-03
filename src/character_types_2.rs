use std::mem::size_of;

fn main() {

    println!("Size of a char: {}", std::mem::size_of::<char>());
    println!("SIze of a char : {}", size_of::<char>());
    // .len == length() -> byte 수
    println!("sSize of string containing 'a' : {}", "a".len());
    println!("sSize of string containing '안녕' : {}", "안녕".len()); // byte 수 -> 결과 : 6

    //글자 수를 알아내는 법
    println!("안녕하세요! count is {}  characters.", "안녕하세요".chars().count());

    let slice = "안녕!";

    println!("Slice is {} bytes but only {} characters.", slice.len(), slice.chars().count());
}