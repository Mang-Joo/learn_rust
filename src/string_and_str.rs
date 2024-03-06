fn main() {
    // String
    // Sized type
    // Data가 heap에 존재하고, 사이즈를 알 수 있다.

    // &str ref str "string slice" 언제까지 데이터를 유지하는지 까지 생각해야돼서 String을 쓰는게 더 편하다.
    // &str = dynamic type
    // "David"는 5바이트일지 6바이트일지 모름
    // 그래서 메모리로 어디부터 어디까지 가지고 있는지로 확인할 수 있음.

    let my_name = "David"; // &str
    // let my_name = "David".to_string(); // String
    let other_name = String::from("Mang_Joo");
    // growable + shrinkable
    let mut my_other_name = "JinAh".to_string();
    my_other_name.push('!');

    println!("{my_other_name}");

    println!("new name is {}", add_new_name(my_name));
}

fn add_new_name(name: &str) -> String {
    let new = String::from(name.clone());

    format!("{new} Hello")
}