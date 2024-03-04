fn main() {
    let my_number = 9.6;
    /// f32 4 bytes
    /// f64 8 bytes 대부분은 64bits
    /// 두 가지가 있다.

    let other_number = 9;

    println!("{}", my_number + other_number as f64);
    println!("{}", my_number as i32 + other_number);
}