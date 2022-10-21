fn main() {
    println!("Chapter pattern match.")
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_1() {
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        if let Some(color) = favorite_color {
            println!("favorite color {}", color);
        } else if is_tuesday {
            println!("is tuesday");
        } else if let Ok(age) = age {
            if age > 30 {
                println!("age over 30")
            } else {
                println!("age not over 30")
            }
        } else {
            println!("Nothing matched")
        }
    }

    #[test]
    fn test_2() {
        let mut stack = vec![];
        stack.push(1);
        stack.push(2);
        stack.push(3);

        while let Some(val) = stack.pop() {
            println!("{}", val);
        }
    }

    #[test]
    fn test_3() {
        let v = vec!['a', 'b', 'c'];

        for (index, value) in v.iter().enumerate() {
            println!("{}: {}", index, value);
        }
    }
}
