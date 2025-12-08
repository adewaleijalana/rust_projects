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

    let blender_orders: Vec<_> = orders
        .iter()
        .filter(|order| order.get_product().eq(&Product::Blender))
        .collect();

    println!("{blender_orders:#?}");

    let total_microwave = orders
        .iter()
        .filter(|order| order.get_product().eq(&Product::Microwave))
        .map(|order| order.get_quantity())
        .sum::<u32>();

    println!("sum of the quantity of Microwave: {total_microwave}");

    let microwave_count = orders
        .iter()
        .filter_map(|order| {
            if order.get_product().eq(&Product::Microwave) {
                Some(order.get_quantity())
            } else {
                None
            }
        })
        .sum::<u32>();

    println!("sum of the quantity of Microwave using filter_map: {microwave_count}");

    let mut envs_arg = env::args().skip(1).take(1);

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
        HashMap::new(),
        |mut data, order| {
            *data.entry(order.get_product()).or_insert(0) += order.get_quantity();

            // if acc.contains_key(&element.get_product()) {
            //     println!("{}", acc[&element.get_product()]);
            //     acc.insert(
            //         &element.get_product(),
            //         acc[&element.get_product()] + element.get_quantity(),
            //     );
            // } else {
            //     acc.insert(element.get_product(), element.get_quantity());
            // }

            data
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
        .into_iter()
        .zip(orders)
        .fold(
            HashMap::<u32, Vec<CustomerOrder>>::new(),
            |mut ids_to_order, (customer_id, order)| {
                // if ids_to_order.contains_key(customer_id) {
                //     let a = ids_to_order.get_mut(customer_id).unwrap();
                //     a.push(*order);
                // } else {
                //     let mut orders = vec![];
                //     orders.push(*order);
                //     ids_to_order.insert(*customer_id, orders);
                // }

                let orders = ids_to_order.entry(customer_id).or_insert(vec![]);
                orders.push(order);
                ids_to_order
            },
        )
        .into_iter()
        .map(|(key, value)| Customer::new(key, value.clone()))
        .collect();

    customers.sort_by_key(|customer| customer.get_customer_id());

    println!("{customers:#?}");
}
