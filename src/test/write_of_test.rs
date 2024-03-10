fn main() {

}
#[cfg(test)]
mod tests {
    use crate::{greeting, Guess, Rectangle};

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert_eq!(larger.can_hold(smaller), true);
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, 2 + 2);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Hello"),
            "Greeting did not conatain name, value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String>{
        if 2+ 2 == 4 {
            Ok(())
        } else {
            Err(String::from("Two plus two does not equal four"))
        }
    }
}

pub fn greeting(name: &str) -> String {
    // format!("Hello {}!", name)
    String::from("Hello")
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

impl Rectangle {
    fn can_hold(&self, other: Rectangle) -> bool {
        self.width > other.width && self.width > other.height
    }
}