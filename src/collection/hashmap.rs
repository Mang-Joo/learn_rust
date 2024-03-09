use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    map.insert(String::from("Blue"), 100);
    map.insert(String::from("Red"), 150);

    let team_name = String::from("Blue");
    let score = map.get(&team_name).copied().unwrap_or(0);

    println!("{team_name} score is {score}");

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // println!("field name is {field_name}");

    let value = map.get("Favorite color").unwrap();

    println!("new value is {value}");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 50);

    scores.entry(String::from("Blue")).or_insert(30);

    println!("{:?}", scores);

    let mut scores = HashMap::new();

    let val = String::from("Hello world world world Hello!");

    for x in val.split_whitespace() {
        let count = scores.entry(x).or_insert(0);
        *count += 1;
    }

    println!("score is {:?}", scores);
}