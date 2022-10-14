use std::cmp;
use std::cmp::PartialOrd;
use std::fmt::Display;

pub struct MinHeap<T: Display + PartialOrd + Copy + Ord> {
    elements: Vec<T>,
}

impl<T: Display + PartialOrd + Copy + Ord> MinHeap<T> {
    pub fn new() -> Self {
        println!("Creating a new empty MinHeap");
        MinHeap {
            elements: Vec::<T>::new(),
        }
    }

    pub fn from(elements: &[T]) -> Self {
        println!("Creating a MinHeap from elements.len()={}", elements.len());
        let mut heap = MinHeap {
            elements: Vec::<T>::with_capacity(elements.len()),
        };
        for item in elements.iter() {
            heap.insert(*item);
        }
        heap
    }

    pub fn insert(&mut self, element: T) {
        self.elements.push(element);

        let mut inserted_element_index = self.elements.len() - 1;
        let mut current_parent_index = inserted_element_index / 2;

        while self.elements[inserted_element_index] < self.elements[current_parent_index] {
            // println!(
            //     "Swapping inserted element {} at index {} with its parent {} at index {}",
            //     self.elements[inserted_element_index],
            //     inserted_element_index,
            //     self.elements[current_parent_index],
            //     current_parent_index
            // );

            self.elements
                .swap(inserted_element_index, current_parent_index);

            // we swapped values of inserted element with its parent,
            // so let's update the indices
            inserted_element_index = current_parent_index;
            current_parent_index = inserted_element_index / 2;
        }
    }

    pub fn extract_min(&mut self) -> Option<T> {
        if self.elements.is_empty() {
            return None;
        }

        let len = self.elements.len();
        // swap root with the last element, remove and return it (to optimize for array shifting)
        self.elements.swap(0, len - 1);
        let min_element = self.elements.remove(len - 1);
        // then restore heap property

        let mut inserted_element_index = 0;
        let mut left_child_index = 1;
        let mut right_child_index = 2;
        // no children = heap property restored
        // only left child = check if > than child, swap if yes and process further down
        // both left and right children exist -> check against each child and swap with the smallest one
        loop {
            let mut smallest_item_index = inserted_element_index;

            if left_child_index < self.elements.len()
                && self.elements[left_child_index] < self.elements[smallest_item_index]
            {
                smallest_item_index = left_child_index;
            }

            if right_child_index < self.elements.len()
                && self.elements[right_child_index] < self.elements[smallest_item_index]
            {
                smallest_item_index = right_child_index;
            }

            if smallest_item_index == inserted_element_index {
                // heap property restored, i.e. inserted element is <= each of its children (or there is no children)
                break;
            }

            self.elements
                .swap(inserted_element_index, smallest_item_index);
            inserted_element_index = smallest_item_index;
            left_child_index = 2 * inserted_element_index;
            right_child_index = 2 * inserted_element_index + 1;
        }

        return Some(min_element);
    }

    pub fn get_min(&self) -> Option<T> {
        if !self.elements.is_empty() {
            return Some(self.elements[0]);
            // let mut max = self.elements[0];
            // for item in self.elements.iter() {
            //     max = cmp::max(max, *item);
            // }
            // return Some(max);
        }
        None
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }
}

struct MaxHeap<T: Display + PartialOrd + Ord> {
    elements: Vec<T>,
}

impl<T: Display + PartialOrd + Copy + Ord> MaxHeap<T> {
    pub fn new() -> Self {
        println!("Creating a new empty MaxHeap");
        MaxHeap {
            elements: Vec::<T>::new(),
        }
    }

    pub fn from(elements: &[T]) -> Self {
        println!("Creating a MaxHeap from elements.len()={}", elements.len());
        let mut heap = MaxHeap {
            elements: Vec::<T>::with_capacity(elements.len()),
        };
        for item in elements.iter() {
            heap.insert(*item);
        }
        heap
    }

    pub fn insert(&mut self, element: T) {
        self.elements.push(element);

        let mut inserted_element_index = self.elements.len() - 1;
        let mut current_parent_index = inserted_element_index / 2;

        while self.elements[inserted_element_index] > self.elements[current_parent_index] {
            // println!(
            //     "Swapping inserted element {} at index {} with its parent {} at index {}",
            //     self.elements[inserted_element_index],
            //     inserted_element_index,
            //     self.elements[current_parent_index],
            //     current_parent_index
            // );

            self.elements
                .swap(inserted_element_index, current_parent_index);

            // we swapped values of inserted element with its parent,
            // so let's update the indices
            inserted_element_index = current_parent_index;
            current_parent_index = inserted_element_index / 2;
        }
    }

    pub fn extract_max(&mut self) -> Option<T> {
        if self.elements.is_empty() {
            return None;
        }

        let len = self.elements.len();
        // swap root with the last element, remove and return it (to optimize for array shifting)
        self.elements.swap(0, len - 1);
        let max_element = self.elements.remove(len - 1);
        // then restore heap property

        let mut inserted_element_index = 0;
        let mut left_child_index = 1;
        let mut right_child_index = 2;
        // no children = heap property restored
        // only left child = check if > than child, swap if yes and process further down
        // both left and right children exist -> check against each child and swap with the smallest one
        loop {
            let mut biggest_item_index = inserted_element_index;

            if left_child_index < self.elements.len()
                && self.elements[left_child_index] > self.elements[biggest_item_index]
            {
                biggest_item_index = left_child_index;
            }

            if right_child_index < self.elements.len()
                && self.elements[right_child_index] > self.elements[biggest_item_index]
            {
                biggest_item_index = right_child_index;
            }

            if biggest_item_index == inserted_element_index {
                // heap property restored, i.e. inserted element is <= each of its children (or there is no children)
                break;
            }

            self.elements
                .swap(inserted_element_index, biggest_item_index);
            inserted_element_index = biggest_item_index;
            left_child_index = 2 * inserted_element_index;
            right_child_index = 2 * inserted_element_index + 1;
        }

        return Some(max_element);
    }

    pub fn get_max(&self) -> Option<T> {
        if !self.elements.is_empty() {
            return Some(self.elements[0]);
            // let mut min = self.elements[0];
            // for item in self.elements.iter() {
            //     min = cmp::min(min, *item);
            // }
            // return Some(min);
        }
        None
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }
}

pub fn heap_sort<T: Display + PartialOrd + Ord + Copy>(arr: &[T]) -> Vec<T> {
    let mut heap = MinHeap::from(arr);

    // for item in arr.iter() {
    //     heap.insert(*item);
    // }

    let mut new_arr = Vec::with_capacity(arr.len());
    while let Some(min_element) = heap.extract_min() {
        new_arr.push(min_element);
    }

    new_arr
}

pub fn compute_median_stream(arr: &[u64]) -> Vec<u64> {
    if arr.is_empty() {
        return vec![];
    } else if arr.len() == 1 {
        return vec![arr[0]];
    }
    // Invariant 1: both h1 and h2 are balanced (same lengths, or one is 1 more than the other at max)
    // Invariant 2: every element of h1 is smaller than every element of h2
    let mut h1 = MaxHeap::<u64>::new();
    let mut h2 = MinHeap::<u64>::new();

    let mut medians = Vec::<u64>::with_capacity(arr.len());

    if arr[0] < arr[1] {
        h1.insert(arr[0]);
        h2.insert(arr[1]);
        medians.push(arr[0]);
        medians.push(arr[0]);
    } else {
        h1.insert(arr[1]);
        h2.insert(arr[0]);
        medians.push(arr[0]);
        medians.push(arr[1]);
    }

    // println!("index=0, item={}", arr[0]);
    // println!("index=1, item={}", arr[1]);

    for index in 2..arr.len() {
        let item = arr[index];
        // println!("index={}, item={}", index, item);

        let h1_max = h1.get_max().unwrap();
        let h2_min = h2.get_min().unwrap();

        if item < h1_max {
            h1.insert(item);
        } else if item > h2_min {
            h2.insert(item);
        } else {
            h1.insert(item);
        }

        let h1_len: i64 = h1.len() as i64;
        let h2_len: i64 = h2.len() as i64;
        if (h1_len - h2_len) > 1 {
            let h1_max = h1.extract_max().unwrap();
            h2.insert(h1_max);
        } else if (h2_len - h1_len) > 1 {
            let h2_min = h2.extract_min().unwrap();
            h1.insert(h2_min);
        }

        let mut median;
        if h1.len() == h2.len() {
            let h1_max = h1.get_max().unwrap();
            // let h2_min = h2.extract_min().unwrap();
            // median = (h1_max + h2_min) / 2;
            median = h1_max;
        } else if h1.len() > h2.len() {
            median = h1.get_max().unwrap();
        } else {
            median = h2.get_min().unwrap();
        }

        medians.push(median);
    }

    println!("medians.len()={}", medians.len());
    println!("medians[..5]={:?}", &medians[..5]);
    let mut median_sum = 0;
    for median in &medians {
        median_sum += median;
    }
    println!("median_sum={}", &median_sum);
    median_sum = median_sum % 10000;
    println!("median_sum % 10000={}", &median_sum);

    // let max = h2.extract_max().unwrap();
    // println!("max={}", max);
    println!(
        "h1.len()={}, h1_max_root={}",
        h1.len(),
        h1.get_max().unwrap()
    );
    println!(
        "h2.len()={}, h2_min_root={}",
        h2.len(),
        h2.get_min().unwrap()
    );

    medians
}
