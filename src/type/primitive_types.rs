fn main() {
    /// signed integer
    /// 부호가 있을 수 있다.
    /// i8, i16, i32, i64, i128
    let integer: u8 = 10; // u8 255까지 가능
    /// unsigned integer
    /// 부호가 없다 (- +가 존재하지않음 like 절대값)
    let unsignedInteger = 5; // i32

    /// 뒤에 붙은 숫자는 bit
    /// 즉 i8 -> 1byte integer
    /// 또한 32비트와 64비트에 따라서 값이 달라진다.
    /// isize -> 32비트 -> i32
    /// isize -> 64비트 -> i64

    // u8 + i32 는 불가능
    let thirdNumber = unsignedInteger;


    let my_number = 9_u8;
    let other_number = 1_000_000_000_u64;
}