fn main() {
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
    //---------
    let rect1 = (30, 50);

    println!(
        "The area of the tuple rectangle is {} square pixels.",
        tuple_area(rect1)
    );

    //-------------------------
    let rect = Rectangle {
        width: dbg!(30 * 2),
        height: 50,
    };

    println!(
        "The area of the struct rectangle is {} square pixels.",
        struct_area(&rect)
    );

    println!("rect is {rect:#?}");
}

fn struct_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn area(width: i32, height: i32) -> i32 {
    width * height
}

fn tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}