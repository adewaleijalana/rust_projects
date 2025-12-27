#[derive(Debug)]
pub enum BinarySearchTree {
    Empty,
    Node {
        value: i32,
        left: Box<BinarySearchTree>,
        right: Box<BinarySearchTree>,
    },
}


impl BinarySearchTree {
    pub fn new() -> Self{
        BinarySearchTree::Empty
    }
}