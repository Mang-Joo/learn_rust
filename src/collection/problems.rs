use std::collections::HashMap;

/// 정수 리스트가 주어졌을 때, 벡터를 이용하여 이 리스트의 중간값 (median, 정렬했을 때 가장 가운데 위치한 값),
/// 그리고 최빈값 (mode, 가장 많이 발생한 값; 해시맵이 여기서 도움이 될 것입니다) 을 반환해 보세요.

/// 문자열을 피그 라틴 (pig Latin) 으로 변경해 보세요. 각 단어의 첫 번째 자음은 단어의 끝으로 이동하고 ‘ay’를 붙이므로,
/// ‘first’는 ‘irst-fay’가 됩니다. 모음으로 시작하는 단어는 대신 끝에 ‘hay’를 붙입니다.
/// (‘apple’은 ‘apple-hay’가 됩니다.) UTF-8 인코딩에 대한 세부 사항을 명심하세요!

/// 해시맵과 벡터를 이용하여 사용자가 회사 부서의 직원 이름을 추가할 수 있도록 하는 텍스트 인터페이스를 만들어 보세요.
/// 예를 들어 ‘Add Sally to Engineering’이나 ‘Add Amir to Sales’ 같은 식으로요.
/// 그 후 사용자가 모든 사람에 대해 알파벳 순으로 정렬된 목록이나 부서별 모든 사람에 대한 목록을 조회할 수 있도록 해보세요.
///

struct Employee {
    dept: String,
    name: String,
}

fn main() {
    let mut db: HashMap<String, String> = HashMap::new();
    // let value = first_question(vec![5, 5, 5, 5, 1, 2, 19, 42, 510, 213, 30, 5, 23, 59, 310]);
    // second_question("apple");
    //
    // println!("first question median is {} and mode is {}", value.0, value.1);
    third_question(String::from("Add abc to engineering"), &mut db);

    println!("{:?}", db);
}

fn second_question(str_value: &str) {
    fn is_vowel(c: char) -> bool {
        matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
    }

    fn pig_latin(word: &str) -> String {
        let mut chars = word.chars();
        match chars.next() {
            Some(first_char) if is_vowel(first_char) => format!("{}-hay", word),
            Some(first_char) => format!("{}-{}ay", chars.collect::<String>(), first_char),
            _other => String::new(),
        }
    }

    let value = pig_latin(str_value);

    println!("value is {}", value);
}

fn first_question(mut vector_value: Vec<i32>) -> (i32, i32) {
    vector_value.sort();

    println!("vector is {:?}", vector_value);

    let median = match vector_value.len() {
        0 => None,
        len if len % 2 == 0 => {
            None
        }
        len => Some(vector_value[len / 2]),
    };

    let mut map = HashMap::new();
    for int_value in vector_value {
        let count = map.entry(int_value).or_insert(0);
        *count += 1;
    }

    let mut mode_key = 0;
    let mut mode_value = 0;

    for (&x, &y) in &map {
        if y > mode_value {
            mode_key = x;
            mode_value = y;
        }
    }


    (median.unwrap(), mode_key)
}

fn third_question(sql: String, db: &mut HashMap<String, String>) -> () {
    let split_sql: Vec<&str> = sql.split_whitespace().collect();
    println!("{}", sql);

    if split_sql.len() != 4 {
        println!("Failed to add Please check sql");
        return;
    }

    if split_sql[0].to_lowercase() == "add" && split_sql[2].to_lowercase() == "to" {
        db.entry(split_sql[1].to_string()).or_insert(split_sql[3].to_string());
    } else {
        println!("Failed to add");
    }
}