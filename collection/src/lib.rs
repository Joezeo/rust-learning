pub mod problem {
    use std::collections::HashMap;

    pub fn problem1() {
        println!("problem 1 :");

        let mut v = vec![1, 2, 3, 4, 2, 32, 11, 32, 3, 4, 2, 3, 4, 22, 33, 22, 2, 4, 4];
        let sum: i32 = v.iter().sum();
        let avg = sum / v.len() as i32;

        v.sort();
        let middle = v[v.len() / 2];

        let mut map = HashMap::new();
        for i in v {
            let c = map.entry(i).or_insert(0);
            *c += 1;
        }

        println!("{:?}", map);
        let mut vect: Vec<(_, _)> = map.iter().collect();
        vect.sort_by(|a, b| b.1.cmp(a.1));
        let mul = match vect.get(0) {
            None => -1,
            Some(c) => *c.0,
        };

        println!("{}", avg);
        println!("{}", middle);
        println!("{}", mul);
    }

    pub fn problem2() {
        println!("problem 2 :");

        let vowel = "-hay";
        let cons = "-ay";
        let strs = vec!["apples", "milk", "cheese", "pie", "rust", "build", "often"];
        for s in strs {
            let mut ss = String::from(s);
            let c = s.chars().next().unwrap();
            let mut flag = true;
            for csd in ['a', 'e', 'i', 'o', 'u'] {
                if c == csd {
                    ss += vowel;
                    flag = false;
                    break;
                }
            }
            if flag {
                ss += cons;
            };
            println!("{}", ss)
        }
    }

    pub mod problem3 {
        use std::collections::HashMap;

        use lazy_static::lazy_static;

        lazy_static! {
            static ref COMPANY: HashMap<String, String> = HashMap::new();

            // TODO: 如何创建一个可变的Vec?
            static ref GROUP: HashMap<String, Vec<String>> = {
                let mut map:HashMap<String, Vec<String>> = HashMap::new();
                map.insert("group1".to_string(), vec![]);
                map.insert("group2".to_string(), vec![]);
                map.insert("group3".to_string(), vec![]);
                map
            };
        }

        pub fn insert(user: String, group: String) {}

        pub fn get_group(group: String) -> Vec<String> {
            vec![]
        }
    }
}