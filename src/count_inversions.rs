// O(n^2)
pub fn count_inv(arr: &[u64]) -> u64 {
    let mut num_inv = 0u64;
    let n = arr.len();
    for i in 0..n - 1 {
        for j in i + 1..n {
            if arr[i] > arr[j] {
                num_inv += 1;
            }
        }
    }
    num_inv
}

// O(n * log n)
pub fn sort_and_count_inv(arr: &[u64]) -> (Vec<u64>, usize) {
    // base case
    if arr.is_empty() {
        return (Vec::<u64>::new(), 0);
    }
    if arr.len() == 1 {
        return (vec![arr[0]], 0);
    }

    let mid = arr.len() / 2;
    let (c, left_inv) = sort_and_count_inv(&arr[..mid]);
    let (d, right_inv) = sort_and_count_inv(&arr[mid..]);
    let (b, split_inv) = merge_and_count_split_inv(&arr);

    (b, left_inv + right_inv + split_inv)
}

fn merge_and_count_split_inv(arr: &[u64]) -> (Vec<u64>, usize) {
    if arr.is_empty() {
        return (Vec::<u64>::new(), 0);
    }
    if arr.len() == 1 {
        return (vec![arr[0]], 0);
    }

    let len = arr.len();
    let half = len / 2;

    let (left, left_split_inv) = merge_and_count_split_inv(&arr[..half]);
    let (right, right_split_inv) = merge_and_count_split_inv(&arr[half..]);

    let mut i = 0;
    let mut j = 0;
    let left_len = left.len();
    let right_len = right.len();
    let mut split_inv: usize = 0;
    let mut tmp: Vec<u64> = Vec::with_capacity(len);
    while i < left_len && j < right_len {
        if left[i] < right[j] {
            tmp.push(left[i]);
            i += 1;
        } else {
            split_inv += half - i;
            // println!("{}-{}, split inversions={}", left[i], right[j], split_inv);
            tmp.push(right[j]);
            j += 1;
        }
    }

    if i < left.len() {
        tmp.extend_from_slice(&left[i..]);
    }
    if j < right.len() {
        tmp.extend_from_slice(&right[j..]);
    }

    (tmp, split_inv)
}
