fn main() {
    let _vec:Vec<i32> = Vec::new();
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
}
