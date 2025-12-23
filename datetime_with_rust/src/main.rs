#![allow(unused)]

use chrono::{NaiveDate, TimeDelta};
use std::ops::{Add,Sub};
fn main() {

    let date = NaiveDate::from_ymd_opt(2025, 12, 23);

    let date = date.unwrap();

    println!("{date:?}");

    let date_plus_5_days = date.add(TimeDelta::days(5));

    println!("{date_plus_5_days:?}");
}
