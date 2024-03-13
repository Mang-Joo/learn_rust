fn main() {
    let array = vec![1, 2, 3];
    let array2 = vec![1, 2, 3];
    let mut array_iter = array.iter();
    let mut array_iter2 = array2.iter();

    let first = array_iter.next();
    let second = array_iter.next();
    let third = array_iter.next();

    println!("{:?}", first.unwrap());
    println!("{:?}", second.unwrap());
    println!("{:?}", third.unwrap());

    let mut into_iter = array.into_iter();

    let first = into_iter.next();
    let second = into_iter.next();
    let third = into_iter.next();

    println!("{:?}", first.unwrap());
    println!("{:?}", second.unwrap());
    println!("{:?}", third.unwrap());

    // println!("{:?}", array); -> Error
    // 소유권이 into_iter()에 넘어감

    let sum_value: i32 = array_iter2.sum();
    println!("{}", sum_value);
    // let sum_value2 = array_iter2.sum(); -> Error
    // iter 소유권이 sum()으로 넘어가서 더 이상 사용할 수 없음.
}