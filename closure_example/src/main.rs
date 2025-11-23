#![allow(dead_code, unused_imports)]

mod chopping;
mod functions;
mod location;

use functions::*;
use location::{Location, Map};

use crate::chopping::{
    shopping_cart::{self, ShoppingCart},
    super_market_item::SupermarketItem,
};

fn main() {
    // borrows_mutably_ex();
    // borrows_immutably_ex();
    // closure_move_ex();

    // unwrap_or_else_test_option();
    // unwrap_or_else_test_result();

    // let closure_method = || println!("Closure Method");

    // closure_function(closure_method)

    // chopping_ex();

    let movie = String::from("Die Hard");
    println!("{}", test(|| movie));
}

fn location_example() {
    let locations = vec![
        Location::new(String::from("Test1"), 5),
        Location::new(String::from("Test2"), 13),
    ];

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

fn chopping_ex() {
    let mut shopping_cart = ShoppingCart::new(vec![
        SupermarketItem::new("APPLE".to_string(), 3.99),
        SupermarketItem::new("BANANA".to_string(), 2.99),
    ]);

    shopping_cart.traverse_items(|item| {
        let new_price = item.price() * 0.85;
        item.set_price(new_price);
    });

    // dbg!(&shopping_cart);

    shopping_cart.traverse_items(|item| {
        item.set_name(item.name().to_lowercase());
    });

    // dbg!(&shopping_cart);

    let mut total_price = 0.0;

    shopping_cart.checkout(|mut shopping_cart| {
        println!("{:#?}", shopping_cart);
        shopping_cart.traverse_items(|item| {
            total_price += item.price();
        });
    });

    println!("Total price: ${:.2}", total_price)
}


fn test<F>(f: F) -> String
where
    F: FnOnce() -> String,
{
    f().to_uppercase()
}
