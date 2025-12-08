#![allow(dead_code, unused)]

use std::{collections::HashMap, env, f32::consts::E};

use crate::{
    customers::{customer::Customer, customer_order::CustomerOrder},
    products::product::Product,
};

mod customers;
mod products;
fn main() {
    let mut orders = vec![
        CustomerOrder::new(Product::Blender, 3, false),
        CustomerOrder::new(Product::Microwave, 1, true),
        CustomerOrder::new(Product::Toaster, 2, false),
        CustomerOrder::new(Product::Microwave, 5, true),
        CustomerOrder::new(Product::Blender, 1, false),
        CustomerOrder::new(Product::Fridge, 10, false),
    ];

    let customer_ids_by_order = [2, 1, 2, 3, 4, 1];

    let results: Vec<_> = orders
        .iter()
        .filter(|order| order.get_product().eq(&Product::Blender))
        .collect();

    // println!("{results:#?}");

    let sum = orders
        .iter()
        .filter(|order| order.get_product().eq(&Product::Microwave))
        .map(|order| order.get_quantity())
        .sum::<u32>();

    println!("sum of the quantity of Microwave: {sum}");

    let sum_2 = orders
        .iter()
        .filter_map(|order| {
            if order.get_product().eq(&Product::Microwave) {
                Some(order.get_quantity())
            } else {
                None
            }
        })
        .sum::<u32>();

    println!("sum of the quantity of Microwave using filter_map: {sum_2}");

    let mut envs_arg = env::args().skip(1);

    // println!("args: {envs_arg:?}");

    let filter_var = envs_arg
        .next()
        .unwrap_or("2".to_string())
        .parse::<u32>()
        .unwrap();

    let results: Vec<_> = orders
        .iter()
        .filter(|order| order.get_quantity() >= filter_var)
        .collect();

    // println!("{results:#?}");

    let results = orders.iter().filter(|order| !order.get_is_shipped()).fold(
        HashMap::<&Product, u32>::new(),
        |mut acc, element| {
            *acc.entry(&element.get_product()).or_insert(0) += element.get_quantity();

            // if acc.contains_key(&element.get_product()) {
            //     println!("{}", acc[&element.get_product()]);
            //     acc.insert(
            //         &element.get_product(),
            //         acc[&element.get_product()] + element.get_quantity(),
            //     );
            // } else {
            //     acc.insert(element.get_product(), element.get_quantity());
            // }

            acc
        },
    );

    println!("{results:#?}");

    let shipped_result = orders
        .iter()
        .find(|order| !order.get_is_shipped())
        .map(|order| CustomerOrder::new(*order.get_product(), order.get_quantity(), true))
        .unwrap();

    // println!("{shipped_result:#?}");

    let mut customers: Vec<_> = customer_ids_by_order
        .iter()
        .zip(orders.iter())
        .fold(HashMap::<i32, Vec<CustomerOrder>>::new(), |mut acc, ele| {
            if acc.contains_key(ele.0) {
                let a = acc.get_mut(ele.0).unwrap();
                a.push(*ele.1);
            } else {
                let mut orders = vec![];
                orders.push(*ele.1);
                acc.insert(*ele.0, orders);
            }
            acc
        })
        .iter()
        .map(|(key, value)| Customer::new(*key as u32, value.clone()))
        .collect();

    customers.sort_by_key(|customer| customer.get_customer_id());

    println!("{customers:#?}");
}
