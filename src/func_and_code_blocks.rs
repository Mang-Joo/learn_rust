fn give_number(one: i32, two: i32) -> i32 {
    one * two
}

fn give_number_empty_tuple(one: i32, two: i32) {
    let multiple = one * two;
    println!("multiple is {multiple}");

    let multiplied_by_ten = {
        let first_number = 10;
        first_number * one * two
    };

    println!("multiplied by ten is {multiplied_by_ten}");
}

fn main() {
    let my_number = give_number(9, 8);
    println!("My number is {my_number}");

    give_number_empty_tuple(7, 8);
}