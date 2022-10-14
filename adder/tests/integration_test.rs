use adder::*;
mod common;

#[test]
fn test_add_two() {
    common::setup();
    let result = add_two(2);
    assert_eq!(result, 4);
}