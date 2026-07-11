use crate::enchanter::Enchanter;

#[derive(Debug)]
pub struct Human {}

impl Enchanter for Human {
    fn competency(&self) -> f64 {
        0.8
    }
}
