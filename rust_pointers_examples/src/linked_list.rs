#[derive(Debug)]
pub enum LinkedList {
    Empty,
    Node { value: i32, next: Box<LinkedList> },
}

impl LinkedList {
    pub fn new(linked_list: LinkedList) -> Self {
        match linked_list {
            LinkedList::Empty => Self::Empty,
            LinkedList::Node { value, next } => Self::Node { value, next },
        }
    }
}
