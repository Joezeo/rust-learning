use std::collections::HashMap;
use collection::problem;
use collection::problem::problem3;

fn main() {
    demo();
    problem::problem1();
    problem::problem2();

    problem3::insert("Joe", problem3::LEARNING_GROUP);
    problem3::insert("Marry", problem3::FOOD_GROUP);
    problem3::insert("Jack", problem3::MUSIC_GROUP);
    problem3::insert("Dan", problem3::MUSIC_GROUP);

    println!("---\nproblem 3 :");
    println!("Music group has people: {:?}", match problem3::get_group_member(problem3::MUSIC_GROUP) {
        Some(gp) => gp,
        None => Vec::new()
    });

    println!("Joe was in group {}", problem3::get_group("Joe"));
}

fn demo() {
    // Vec
    let _vec: Vec<i32> = Vec::new();
    let mut v = vec![1u32, 2, 3, 4, 5];
    v.push(6);
    v.push(7);

    let _v3 = v[3];
    match v.get(3) {
        Some(x) => println!("{}", x),
        None => println!("None found"),
    }

    let pan = &v[4];
    println!("{}", pan);
    v.push(66);

    for i in &mut v {
        *i += 50;
    }

    for i in v {
        println!("{}", i);
    }

    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f32),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(0),
        SpreadSheetCell::Float(1.0),
        SpreadSheetCell::Text(String::from("Hello world!")),
        SpreadSheetCell::Float(1.0),
    ];

    for i in &row {
        println!("{:?}", i);
    }

    // String
    let mut s1 = String::from("Hello");
    let s2 = String::from("world");
    s1.push_str(&s2);
    println!("{}", s1);
    println!("{}", s2);

    let c1 = String::from("tic");
    let c2 = String::from("tac");
    let c3 = String::from("toe");
    let s = format!("{}-{}-{}", c1, c2, c3);
    println!("{}", s);

    let cs = String::from("你好啊");
    println!("字节数量 {} : 字符数量 {}", cs.len(), cs.chars().count());
    println!("{}", &cs[0..6]);
    for char in cs.chars() {
        println!("{}", char);
    }

    // HashMap
    let mut map = HashMap::new();
    map.insert(String::from("Blue"), 11);
    map.insert(String::from("Blue"), 12);

    let teams = vec![String::from("Blue"), String::from("Yellow"), String::from("Green")];
    let init_scores = vec![10, 30, 25];
    let mut scores: HashMap<_, _> = teams.iter().zip(init_scores).collect();
    match scores.get(&teams[0]) {
        Some(x) => println!("{}", x),
        None => println!("None found"),
    }

    let blue = String::from("Blue");
    scores.insert(&blue, 40);
    println!("{:#?}", scores);

    let mut another_score = HashMap::new();
    another_score.insert(String::from("Blue"), 10);
    another_score.entry(String::from("Blue")).or_insert(50);
    another_score.entry(String::from("Yellow")).or_insert(50);
    println!("{:?}", another_score);

    let words = "White world hello world world hello White is the world";
    let mut map = HashMap::new();
    for wd in words.split_whitespace() {
        let c = map.entry(wd).or_insert(0);
        *c += 1;
    }
    println!("{:?}", map);
}
