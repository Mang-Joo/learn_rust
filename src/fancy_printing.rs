fn main() {
    print!("This\\nis\na\nbunch\nof\nlines");
    println!(r#"C:\thisdrive_\"#); // raw text

    println!(
        "Let me tell you
        어떤 이야기를
        봅시다."
    );

    let my_variable = &9;
    println!("{:p}", my_variable); // 메모리 주소 출력

    let my_number = 14;
    println!("{:b}", my_number); // 2진수 표현
    println!("{:X}", my_number); // 16진수 표현

    println!("{:-^30}", "TODAY'S NEWS");
    let bar = "|";
    println!("{: <15}{: >15}", bar, bar);
    let ko = "SEOUL";
    let jp = "TOKYO";
    println!("{ko:-<15}{jp:->15}");
    let ko = { format!("{ko:h<15}끝") };
    println!("{ko}");
}