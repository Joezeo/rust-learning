pub mod problem {
    use std::collections::HashMap;

    pub fn problem1() {
        println!("---\nproblem 1 :");

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
        println!("---\nproblem 2 :");

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
        use std::sync::Mutex;

        use lazy_static::lazy_static;

        pub const LEARNING_GROUP: &str = "Learning";
        pub const FOOD_GROUP: &str = "Food";
        pub const MUSIC_GROUP: &str = "Music";

        lazy_static! {
            static ref COMPANY: Mutex<HashMap<String, String>> = {
                let map = HashMap::new();
                Mutex::new(map)
            };

            static ref GROUP: Mutex<HashMap<String, Vec<String>>> = {
                let mut map:HashMap<String, Vec<String>> = HashMap::new();
                map.insert(LEARNING_GROUP.to_string(), vec![]);
                map.insert(FOOD_GROUP.to_string(), vec![]);
                map.insert(MUSIC_GROUP.to_string(), vec![]);
                Mutex::new(map)
            };
        }

        pub fn insert(user: &str, group: &str) {
            COMPANY.lock().unwrap().insert(user.to_string(), group.to_string());
            match GROUP.lock().unwrap().get_mut(group) {
                Some(vc) => vc.push(user.to_string()),
                None => {}
            }
        }

        pub fn get_group_member(group: &str) -> Option<Vec<String>> {
            return match GROUP.lock().unwrap().get(group) {
                Some(gp) => Some(gp.clone()),
                None => None
            }
        }

        pub fn get_group(user: &str) -> String {
            return match COMPANY.lock().unwrap().get(user) {
                Some(group) => group.clone(),
                None => "none".to_string()
            }
        }
    }
}