#![allow(unused)]

use chrono::prelude::*;
use chrono_tz::Africa::Lagos;
use std::ops::{Add,Sub};
fn main() {

    // let date = NaiveDate::from_ymd_opt(2025, 12, 23);

    // let date = date.unwrap();

    // println!("{date:?}");

    // let date_plus_5_days = date.add(TimeDelta::days(5));

    // println!("{date_plus_5_days:?}");
    let system_time = Local::now();
    let utc_time = Utc::now();

    println!("{}", system_time.date_naive());

    println!("{}", utc_time.date_naive());

    println!("{}", system_time.date_naive());

    println!("{}", system_time.time());

    println!("{}", utc_time.time());

    println!("{}", system_time.offset());

    println!("{}", utc_time.offset());
}
