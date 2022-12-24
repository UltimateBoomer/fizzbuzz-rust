mod fizzbuzz;

use std::collections::BTreeMap;

fn main() {
    let mut word_map = BTreeMap::new();
    word_map.insert(3, "fizz");
    word_map.insert(5, "buzz");
    word_map.insert(15, "fizzbuzz");

    println!("{:?}", fizzbuzz::list(1, 15, &word_map));
}
