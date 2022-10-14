pub fn insertion_sort<T: PartialOrd>(arr: &mut [T]) {
    let arr_length = arr.len();

    if arr_length < 2 {
        return;
    }

    for i in 1..arr_length {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut arr: [i32; 0] = [];
        insertion_sort(&mut arr);
        assert_eq!(arr, []);

        let mut arr = [1];
        insertion_sort(&mut arr);
        assert_eq!(arr, [1]);

        let mut arr = [1, 2];
        insertion_sort(&mut arr);
        assert_eq!(arr, [1, 2]);

        let mut arr = [2, 1];
        insertion_sort(&mut arr);
        assert_eq!(arr, [1, 2]);

        let mut arr = [2, 2];
        insertion_sort(&mut arr);
        assert_eq!(arr, [2, 2]);

        let mut arr = [0, 0, 0, 0, 0, 0];
        insertion_sort(&mut arr);
        assert_eq!(arr, [0, 0, 0, 0, 0, 0]);

        let mut arr = [10, 5, 0, 2, 1, 3, 4, 6, 9, 7, 8];
        insertion_sort(&mut arr);
        assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        let mut arr = [10, 5, 0, 2, 1, 3, 7, 4, 6, 9, 7, 8];
        insertion_sort(&mut arr);
        assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 7, 7, 8, 9, 10]);

        let mut arr = [10, 5, 0, 2, 1, 3, 7, 4, 6, 9, 7, 10, 8];
        insertion_sort(&mut arr);
        assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 7, 7, 8, 9, 10, 10]);

        let mut arr = [10, 5, 0, 2, 1, 0, 3, 7, 4, 6, 9, 7, 10, 8];
        insertion_sort(&mut arr);
        assert_eq!(arr, [0, 0, 1, 2, 3, 4, 5, 6, 7, 7, 8, 9, 10, 10]);

        let mut arr = [10, 5, 0, 2, 1, 0, 3, 7, 4, 6, 6, 9, 7, 6, 10, 8];
        insertion_sort(&mut arr);
        assert_eq!(arr, [0, 0, 1, 2, 3, 4, 5, 6, 6, 6, 7, 7, 8, 9, 10, 10]);

        let mut vec = vec![0, 3, 3, 6, 8, 2];
        insertion_sort(&mut vec);
        assert_eq!(vec, vec![0, 2, 3, 3, 6, 8]);
    }
}
