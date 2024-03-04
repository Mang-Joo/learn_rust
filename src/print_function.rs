// macro == function that writes code

fn main() {
    println!("Hello, world!");

    let my_name = "Mang_Joo";
    let my_age = give_age();
    println!("My name is {my_name} and my age is {my_age}");
    println!("My name is {0} and my age is {1}", my_name, my_age);
}

fn give_age() -> i32 { 28 }