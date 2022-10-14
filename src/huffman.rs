use crate::heap;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug)]
pub struct BinaryTree<T: Ord> {
    pub value: T,
    pub left: Option<Box<BinaryTree<T>>>,
    pub right: Option<Box<BinaryTree<T>>>,
}

impl PartialEq for BinaryTree<usize> {
    fn eq(&self, other: &BinaryTree<usize>) -> bool {
        self.value == other.value
    }
}

impl Eq for BinaryTree<usize> {}

impl PartialOrd for BinaryTree<usize> {
    fn partial_cmp(&self, other: &BinaryTree<usize>) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for BinaryTree<usize> {
    fn cmp(&self, other: &BinaryTree<usize>) -> Ordering {
        self.value.cmp(&other.value)
    }
}

pub fn compute_code_tree(frequencies: &[usize]) -> BinaryTree<usize> {
    let mut heap = heap::MinHeap::<usize>::new();
    let mut forest = HashMap::<usize, BinaryTree<usize>>::with_capacity(frequencies.len());
    for frequency in frequencies {
        forest.insert(
            *frequency,
            BinaryTree {
                value: *frequency,
                left: None,
                right: None,
            },
        );
        heap.insert(*frequency);
    }
    // forest.sort_by(|a, b| b.value.cmp(&a.value));
    // println!("forst = {:?}", forest);
    assert_eq!(heap.len(), forest.len());

    while forest.len() > 1 {
        // println!("forest.len() = {}, heap.len() = {}", forest.len(), heap.len());

        let min_freq = heap.extract_min().unwrap();
        // println!("min_freq={}", min_freq);
        let min_freq_tree = forest.remove(&min_freq).unwrap();

        let second_min_freq = heap.extract_min().unwrap();
        // println!("second_min_freq={}", second_min_freq);
        let second_min_freq_tree = forest.remove(&second_min_freq).unwrap();

        let merged_frequency = min_freq_tree.value + second_min_freq_tree.value;
        let merged_tree = BinaryTree {
            value: merged_frequency,
            left: Some(Box::new(min_freq_tree)),
            right: Some(Box::new(second_min_freq_tree)),
        };
        heap.insert(merged_frequency);
        forest.insert(merged_frequency, merged_tree);
    }

    // return unique tree remaining in the forest
    let mut code_tree: Vec<BinaryTree<usize>> = forest.into_values().collect();
    code_tree.remove(0)
}
