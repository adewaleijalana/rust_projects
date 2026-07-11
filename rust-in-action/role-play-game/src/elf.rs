use crate::enchanter::Enchanter;

#[derive(Debug)]
pub struct Elf {}

impl Enchanter for Elf {
    fn competency(&self) -> f64 {
        0.95
    }
}
