use std::fmt::{Display, Formatter, Result};

use crate::drinks::drinkable::Drinkable;

#[derive(Debug)]
pub struct Soda {
    calories: u32,
    price: f64,
    flavor: String,
    percentage: u32,
}

impl Soda {
    pub fn new(calories: u32, price: f64, flavor: String, percentage: u32) -> Self {
        Self {
            calories,
            price,
            flavor,
            percentage,
        }
    }
}

impl Drinkable for Soda {
    fn consume(&mut self) {
        self.percentage = 0;
    }

    fn get_data(&self) -> String {
        format!("Flavor: {}, Calories: {}", self.flavor, self.calories)
    }
}

impl Display for Soda {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, "** {} Soda **", self.flavor)
    }
}

impl Clone for Soda {
    fn clone(&self) -> Self {
        Self {
            calories: self.calories.clone(),
            price: self.price.clone(),
            flavor: self.flavor.clone(),
            percentage: self.percentage.clone(),
        }
    }
}

impl PartialEq for Soda {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

impl Eq for Soda {}
