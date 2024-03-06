/// stack
/// 함수 내부에서만 사용하는 영역
/// -> 가장 빠름

/// heap
/// 위치를 찾아갈 수 있게 해주는데 공간이 많고

/// pointers (references)
/// like book
/// 내가 원하는 메모리를 가져올 수 있게 해줌
/// reference (잠깐 빌린다.)

fn main() {
    let my_number = 15; // this is an i32
    let single_reference = &my_number;  s// reference to my_number
    let double_reference = &single_reference; // this is a &&i32 -> 거의 사용하지 않음
    let five_reference = &&&&&my_number; // this is a &&&&&i32 -> 사용 안함.
}