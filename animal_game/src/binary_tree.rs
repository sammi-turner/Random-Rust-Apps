use std::collections::HashMap;
use std::io;

pub struct BinaryTree {
    pub nodes: HashMap<usize,String>,
    pub cursor: usize,
}

impl BinaryTree {
    // Main loop
    pub fn main_loop(&mut self) {
        while self.keep_playing() {
            self.restart();
            while ! self.is_leaf() {
                println!("{}? ", self.get());
                if self.yes_no() {
                    self.yes();
                } else {
                    self.no();
                }
            }
            println!("Is it a {}? ", self.get());
            if ! self.yes_no() {
                println!("The animal you were thinking of was a ? ");
                let new_animal = self.read_input();
                let old_animal = self.get();
                println!("Please type in a question that would distinguish a {} from a {} : ",
                         new_animal, old_animal );
                let new_question = self.read_input();
                println!("For a {}, the answer would be? ", new_animal);
                if self.yes_no() {
                    self.set(new_question, new_animal, old_animal)
                } else {
                    self.set(new_question, old_animal, new_animal)
                }
            }
        }
    }

    // Creates new BinaryTree with one root node (question) and two child animals.
    pub fn new(value: String, yes: String, no: String) -> BinaryTree {
        let nodes: HashMap<usize,String> = HashMap::from([
            (0, value),
            (1, yes),
            (2, no),
        ]);
        BinaryTree { nodes, cursor: 0 }
    }

    // Reads the input line.
    fn read_input(&mut self) -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse::<String>().unwrap()
    }

    // Asks the player whether the player wants to continue playing. Returns true if the answer is yes.
    fn keep_playing(&mut self) -> bool {
        println!("\nAre you thinking of an animal? (enter 'y' or 'n') ");
        return self.yes_no();
    }

    // Checks whether given answer is yes or no.
    fn yes_no(&mut self) -> bool {
        loop {
            let answer = self.read_input();
            if answer.to_lowercase() != "y" && answer.to_lowercase() != "n" {
                println!("Please enter 'y' or 'n'.");
                continue;
            }
            return answer.to_lowercase() == "y";
        }
    }

    // Returns the key for left node (yes) based on the position of the BinaryTree cursor.
    fn get_yes_key(&self) -> usize {
        &self.cursor * 2 + 1
    }

    // Returns the key for right node (no) based on the position of the BinaryTree cursor.
    fn get_no_key(&self) -> usize {
        &self.cursor * 2 + 2
    }

    // Check if current node is a leaf (has no children).
    fn is_leaf(&self) -> bool {
        ! ( self.nodes.contains_key(&self.get_yes_key()) ||
            self.nodes.contains_key(&self.get_no_key()) )
    }

    // Moves cursor to "yes" (left node) if the node exists.
    fn yes(&mut self) {
        if self.nodes.contains_key(&self.get_yes_key()) {
            self.cursor = self.get_yes_key();
        }
    }

    // Moves cursor to "no" (right node) if the node exists.
    fn no(&mut self) {
        if self.nodes.contains_key(&self.get_no_key()) {
            self.cursor = self.get_no_key();
        }
    }

    // Sets new value (question) and two children (animals) at current position of the BinaryTree cursor.
    fn set(&mut self, value: String, yes: String, no: String) {
        if let Some(v) = self.nodes.get_mut(&self.cursor) {
            *v = value;
        }
        self.nodes.insert(self.get_yes_key(), yes);
        self.nodes.insert(self.get_no_key(), no);
    }

    // Returns the value (question or animal) of the current node.
    fn get(&self) -> String {
        if let Some(t) = self.nodes.get(&self.cursor) {
            return t.to_string();
        }
        "".to_string()
    }

    // Reset cursor to 0 (root node).
    fn restart(&mut self) {
        self.cursor = 0;
    }
}