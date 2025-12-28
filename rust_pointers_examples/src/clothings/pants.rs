use std::fmt::format;

use crate::clothings::wearable::Wearable;

#[derive(Debug)]
pub struct Pants {
    fabric: String,
    waist_size: u32,
}

impl Pants {
    pub fn new(fabric: String, waist_size: u32) -> Self {
        Self { fabric, waist_size }
    }
}

impl Wearable for Pants {
    fn wear(&self) -> String {
        format!("{} {} pants", self.waist_size, self.fabric)
    }
}
