fn main() {
    // 문자 하나!
    // char는 4byte
    let first_letter = 'A';

    let space = ' ';
    let other_language_char = '히';
    let cat_face = 'ㄹ';

    // ASCII
    // casting = simple type change using 'as'
    // 확실하게 바꿀 수 있으면 as를 사용하고 아니면 불가능하다.
    let my_number: u16 = 8; // i32
    let second_number: u8 = 10;
    let third_number = my_number + second_number as u16; // -> type casting
    println!("third_number =  {third_number}");
    println!("Hello World! My number is {}", my_number);
}