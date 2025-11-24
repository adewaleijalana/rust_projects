use orphan_rule::{Point, Printable};

struct PointWrapper(Point);

impl Printable for PointWrapper {
    fn print(&self) {
        println!("Point({}, {})", self.0.x, self.0.y);
    }
    
}
fn main() {
    let point = Point { x: 1.0, y: 2.0 };
    let wrapped_point = PointWrapper(point);
    
    wrapped_point.print();
    println!("Wrapped point created and printed successfully.");
}
