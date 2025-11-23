#![allow(dead_code, unused_imports)]

mod functions;
mod location;
use functions::*;
use location::{Location, Map};


fn main() {
    // borrows_mutably_ex();
    // borrows_immutably_ex();
    // closure_move_ex();

    // unwrap_or_else_test_option();
    // unwrap_or_else_test_result();

    // let closure_method = || println!("Closure Method");

    // closure_function(closure_method)

    let locations = vec![Location::new(String::from("Test1"), 5), Location::new(String::from("Test2"), 13)];

    let map = Map::new(&locations);

    let mut total_treasures = 0;

    let mut location_name = Vec::<String>::new();

    map.explore(|location| {
        total_treasures += location.treasure();
    });

    println!("Total number of treasures: {}", total_treasures);

    map.explore(|loc| {
        location_name.push(loc.name());
    });

    println!("Location name: {:?}", location_name);
}