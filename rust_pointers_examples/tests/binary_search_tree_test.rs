use rstest::{fixture, rstest};
use rust_pointers_examples::binary_search_tree::BinarySearchTree;

#[fixture]
fn binary_search_tree_with_three_nodes() -> BinarySearchTree {
    let mut binary_search_tree = BinarySearchTree::new();

    binary_search_tree.insert(5);

    binary_search_tree.insert(2);

    binary_search_tree.insert(8);

    binary_search_tree
}

#[rstest]
fn contains_method_should_return_true(binary_search_tree_with_three_nodes: BinarySearchTree) {

    assert!(binary_search_tree_with_three_nodes.contains(5));
}

#[rstest]
fn contains_method_should_return_false(binary_search_tree_with_three_nodes: BinarySearchTree) {

    assert!(!binary_search_tree_with_three_nodes.contains(15));
}
