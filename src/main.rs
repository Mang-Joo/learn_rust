fn main() {
    let a = Solution::roman_to_int(String::from("MCMXCIV"));

    println!("{a}");
}

struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let s_translated = s
            .replace("IV", "IIII")
            .replace("IX", "VIIII")
            .replace("XL", "XXXX")
            .replace("XC", "LXXXX")
            .replace("CD", "CCCC")
            .replace("CM", "DCCCC");

        s_translated.chars().map(|c| {
            match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _other => 0
            }
        }).sum()
    }
}

enum Roman {
    I,
    V,
    X,
    L,
    C,
    D,
    M
}

impl Roman {
    fn find_value(&self) -> i32 {
        match &self {
            Roman::I => {1}
            Roman::V => {5}
            Roman::X => {10}
            Roman::L => {50}
            Roman::C => {100}
            Roman::D => {500}
            Roman::M => {1000}
        }
    }

    fn find_char(check_value: char) -> Self {
        match check_value {
            'I' => {Roman::I}
            'V' => {Roman::V}
            'X' => {Roman::X}
            'L' => {Roman::L}
            'C' => {Roman::C}
            'D' => {Roman::D}
            'M' => {Roman::M}
            other => {Roman::I}
        }
    }
}