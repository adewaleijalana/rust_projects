use std::cmp::Ordering;

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
    pub fn new() -> Self {
        BinarySearchTree::Empty
    }

    pub fn insert(&mut self, new_value: i32) {
        match self {
            BinarySearchTree::Empty => {
                let new_node = BinarySearchTree::Node {
                    value: new_value,
                    left: Box::new(BinarySearchTree::Empty),
                    right: Box::new(BinarySearchTree::Empty),
                };
                *self = new_node;
            }
            BinarySearchTree::Node { value, left, right } => match new_value.cmp(value) {
                Ordering::Equal => (),
                Ordering::Greater => {
                    right.insert(new_value);
                }
                Ordering::Less => {
                    left.insert(new_value);
                }
            },
        }
    }

    pub fn contains(&self, target: i32) -> bool {
        match self {
            BinarySearchTree::Empty => false,
            BinarySearchTree::Node { value, left, right } => match target.cmp(value) {
                Ordering::Equal => true,
                Ordering::Greater => {
                    right.contains(target)
                }
                Ordering::Less => {
                    left.contains(target)
                }
            },
        }
    }
}
