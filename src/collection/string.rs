fn main() {
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // 이 메서드는 리터럴에서도 바로 작동합니다:
    let s = "initial contents".to_string();

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("Hola");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("你好");

    println!("{hello}");

    //slice

    let hello = "Здравствуйте";

    let s = &hello[0..4];

    println!("s is {s}");
}