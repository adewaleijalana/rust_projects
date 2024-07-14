mod common;
use adder::add_two;
use common::setup;

#[test]
fn it_adds_two() {
    assert_eq!(4, add_two(2));
}