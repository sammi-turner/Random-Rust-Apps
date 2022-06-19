mod binary_tree;
use binary_tree::{BinaryTree};

fn main() {
    println!("Welcome to GUESS THE ANIMAL");

    let mut animal = BinaryTree::new(
        "Does it swim".to_string(),
        "Fish".to_string(),
        "Bird".to_string(),
    );

    BinaryTree::main_loop(&mut animal);
}