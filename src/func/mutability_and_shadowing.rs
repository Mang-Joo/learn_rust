// mutability

// immutable by default
// mut == mutable

// shadowing
// 같은 이름을 다시 쓰는 것

fn main() {
    let mut my_number = 10;
    my_number = 9;

    println!("my number is {my_number}");

    let my_variable = 10;
    let my_variable = 9; // 이거는 새로 만드는 것 (safe 함)

    println!("my variable is {my_variable}");

    let second_variable = 9;
    {
        let second_variable = "Some string";
        println!("{second_variable}");
    }

    println!("{second_variable}");
}