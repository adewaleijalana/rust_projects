#[derive(Debug)]
pub enum LinkedList {
    Empty,
    Node { value: i32, next: Box<LinkedList> },
}


// impl LinkedList {
//     pub fn new() -> Self {
//         Self::Empty
//     }
// }