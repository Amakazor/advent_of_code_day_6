use std::collections::{HashSet, VecDeque};
use std::fs;

fn is_unique(deque: &VecDeque<char>) -> bool {
    let set:HashSet<&char> = HashSet::from_iter(deque.iter());
    return set.len() == deque.len()
}

fn find_index_after_unique(data: &str, amount: usize) -> usize {
    let mut uniques:VecDeque<char> = VecDeque::with_capacity(amount);

    for (index, character) in data.chars().enumerate() {
        if uniques.len() == amount { 
            uniques.pop_back();
        }
        
        uniques.push_front(character);

        if uniques.len() == amount && is_unique(&uniques) {
            return index+1;
        }
    }
    
    panic!("No sequence found")
}

fn main() {
    let data = fs::read_to_string("final-data.txt").unwrap();
    let start_of_signal = find_index_after_unique(data.as_str(), 4);
    let start_of_message = find_index_after_unique(data.as_str(), 14);
    
    println!("Start of signal marker: {}", start_of_signal);
    println!("Start of message marker: {}", start_of_message);
}
