// () - empty tuple, unit type (void)

fn number() -> i32 {
    8
}

fn empty_tuple() { // == empty_tuple () -> () {}
    println!("123")
}

// semicolon 이 있으면 empty tuple 이 나온다.
fn main() {
    let my_number = number();

    let tuple = empty_tuple();

    println!("{my_number}");

    /// {} == Display
    /// {:?} == Debug print
    println!("{:?}", tuple);
    // print => ()
}