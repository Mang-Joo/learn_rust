
fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let direct_vector = vec![1, 2, 3];

    let first = &v[0];

    println!("{v:?}");
    println!("{direct_vector:?}");

    let third: Option<&i32> = direct_vector.get(2);

    match third {
        Some(third) => println!("THe third element is {third}"),
        other => println!("There is no element"),
    };

    for (i) in &direct_vector {
        println!("{i}");
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(10.12),
        SpreadSheetCell::Text(String::from("Blue")),
    ];

    for value in &row {
        println!("value is {value:?}");
    }

    let pop_value = v.pop().unwrap_or_default();
    println!("pop value is {pop_value}");

    println!("v = {v:?}")
}

#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
