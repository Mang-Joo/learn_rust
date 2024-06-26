use crate::List::{Cons, Nil};

fn main() {
    let box_test = Box::new(5);

    println!("b = {}", box_test);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("list = {:?}", list);
}


#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}