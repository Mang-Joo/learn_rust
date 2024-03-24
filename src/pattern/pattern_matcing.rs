fn main() {
    let x = Option::Some(5);

    let x = match x {
        None => panic!("No have value"),
        Some(value) => value + 5
    };
    println!("x value is {}", x);

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Have favorite color! color is {}", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple color");
        } else {
            println!("Using orange color");
        }
    } else {
        println!("Using blue color");
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

}