#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, another: &Rectangle) -> bool {
        self.length > another.length && self.width > another.width
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(num: i32) -> i32 {
    num + 2
}

pub struct Guess {
    _val: i32,
}

impl Guess {
    pub fn new(val: i32) -> Self {
        if val < 0 || val > 100 { panic!("Guess should panic when val > 100 or val < 0!") };
        Guess { _val: val }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        println!("Test exploration success.")
    }

    #[test]
    fn another() {
        let result = add(2, 3);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_can_hold() {
        let bigger = Rectangle { length: 30, width: 20 };
        let smaller = Rectangle { length: 10, width: 10 };
        assert!(bigger.can_hold(&smaller));
        assert!(!smaller.can_hold(&bigger));
    }

    #[test]
    fn test_add_two() {
        let result = add_two(99);
        assert_eq!(result, 101);
    }

    #[test]
    #[should_panic(expected = "Guess should panic when val > 100 or val < 0!")]
    fn test_guess() {
        let _guess = Guess::new(200);
    }

    #[test]
    fn test_result() -> Result<(), String> {
        let result = add(2, 2);
        if result == 4 {
            Ok(())
        } else {
            Err("Result should be 4".to_string())
        }
    }
}
