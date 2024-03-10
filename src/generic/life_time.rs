fn main() {
    let a = String::from("abcd");
    let b = "abc";

    let result = longest(&a, b);

    println!("The longest is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}