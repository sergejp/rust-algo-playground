pub fn merge_sort<T: PartialOrd + Copy + std::fmt::Debug>(arr: &mut [T]) {
    let len = arr.len();

    if len < 2 {
        return;
    }

    let half = len / 2;
    merge_sort(&mut arr[..half]);
    merge_sort(&mut arr[half..]);

    let mut i = 0;
    let mut j = 0;
    let left = &arr[..half];
    let right = &arr[half..];
    let mut tmp: Vec<T> = Vec::with_capacity(len);
    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            tmp.push(left[i]);
            i += 1;
        } else {
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

    arr.copy_from_slice(&tmp[..]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut arr: [i32; 0] = [];
        merge_sort(&mut arr);
        assert_eq!(arr, []);

        let mut arr = [1];
        merge_sort(&mut arr);
        assert_eq!(arr, [1]);

        let mut arr = [1, 2];
        merge_sort(&mut arr);
        assert_eq!(arr, [1, 2]);

        let mut arr = [2, 1];
        merge_sort(&mut arr);
        assert_eq!(arr, [1, 2]);

        let mut arr = [2, 2];
        merge_sort(&mut arr);
        assert_eq!(arr, [2, 2]);

        let mut arr = [0, 0, 0, 0, 0, 0];
        merge_sort(&mut arr);
        assert_eq!(arr, [0, 0, 0, 0, 0, 0]);

        let mut arr = [10, 5, 0, 2, 1];
        merge_sort(&mut arr);
        assert_eq!(arr, [0, 1, 2, 5, 10]);

        let mut arr = [10, 5, 0, 2, 1, 3, 4, 6, 9, 7, 8];
        merge_sort(&mut arr);
        assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        let mut arr = [10, 5, 0, 2, 1, 3, 7, 4, 6, 9, 7, 8];
        merge_sort(&mut arr);
        assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 7, 7, 8, 9, 10]);

        let mut arr = [10, 5, 0, 2, 1, 3, 7, 4, 6, 9, 7, 10, 8];
        merge_sort(&mut arr);
        assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 7, 7, 8, 9, 10, 10]);

        let mut arr = [10, 5, 0, 2, 1, 0, 3, 7, 4, 6, 9, 7, 10, 8];
        merge_sort(&mut arr);
        assert_eq!(arr, [0, 0, 1, 2, 3, 4, 5, 6, 7, 7, 8, 9, 10, 10]);

        let mut arr = [10, 5, 0, 2, 1, 0, 3, 7, 4, 6, 6, 9, 7, 6, 10, 8];
        merge_sort(&mut arr);
        assert_eq!(arr, [0, 0, 1, 2, 3, 4, 5, 6, 6, 6, 7, 7, 8, 9, 10, 10]);

        let mut vec = vec![0, 3, 3, 6, 8, 2];
        merge_sort(&mut vec);
        assert_eq!(vec, vec![0, 2, 3, 3, 6, 8]);
    }
}
