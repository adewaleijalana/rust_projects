use crate::enchanter::Enchanter;

#[derive(Debug)]
pub struct Dwarf {}

impl Enchanter for Dwarf {
    fn competency(&self) -> f64 {
        0.5
    }
}
