use test_macro::TestMacro;
use test_macro_derive::TestMacro;

#[derive(TestMacro)]
struct Bananas;

fn main() {
    Bananas::first_macro();
}