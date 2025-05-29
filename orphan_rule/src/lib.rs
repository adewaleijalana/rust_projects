pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub trait Printable {
    fn print(&self);
}