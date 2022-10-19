macro_rules! foo {
    ($a:expr) => {
        $a
    };
    ($a:expr, $b:expr) => {
        $a + $b
    }
}
fn main() {
    let m = foo!("Hello world");
    println!("{}", m);

    assert_eq!(4, foo!(2, 2));
}
