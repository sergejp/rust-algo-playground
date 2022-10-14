// static mut NUM: usize = 0;

pub fn quick_sort<T: PartialOrd + Copy>(arr: &mut [T], l: usize, r: usize) {
    if l >= r {
        return;
    }

    let i = choose_pivot(arr, l, r);
    arr.swap(l, i);
    let j = partition(arr, l, r);
    if j > 0 {
        // println!("Recursive: {}", 1+j-1-l);
        // unsafe {
        //     NUM += 1+j-1-l;
        //     println!("NUM={}", NUM);
        // }
        quick_sort(arr, l, j - 1);
    }

    // unsafe {
    //     NUM += 1+r-(j+1);
    //     println!("NUM={}", NUM);
    // }
    quick_sort(arr, j + 1, r);
}

fn choose_pivot<T: PartialOrd + Copy>(arr: &mut [T], l: usize, r: usize) -> usize {
    // l
    // r
    // println!("arr.len()={}, l={}, r={}", arr.len(), l, r);
    let mid = l + (r - l) / 2;
    if (arr[l] >= arr[mid] && arr[l] <= arr[r]) || (arr[l] <= arr[mid] && arr[l] >= arr[r]) {
        return l;
    }
    if (arr[mid] >= arr[l] && arr[mid] <= arr[r]) || (arr[mid] <= arr[l] && arr[mid] >= arr[r]) {
        return mid;
    } else {
        return r;
    }
}

fn partition<T: PartialOrd + Copy>(arr: &mut [T], l: usize, r: usize) -> usize {
    let p = arr[l];
    let mut i = l + 1;
    for j in (l + 1)..=r {
        if arr[j] < p {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(l, i - 1);
    i - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut arr: [i32; 0] = [];
        quick_sort(&mut arr, 0, 0);
        assert_eq!(arr, []);

        let mut arr = [1];
        quick_sort(&mut arr, 0, 0);
        assert_eq!(arr, [1]);

        let mut arr = [1, 2];
        quick_sort(&mut arr, 0, 1);
        assert_eq!(arr, [1, 2]);

        let mut arr = [2, 1];
        quick_sort(&mut arr, 0, 1);
        assert_eq!(arr, [1, 2]);

        let mut arr = [2, 2];
        quick_sort(&mut arr, 0, 1);
        assert_eq!(arr, [2, 2]);

        let mut arr = [0, 0, 0, 0, 0, 0];
        quick_sort(&mut arr, 0, 5);
        assert_eq!(arr, [0, 0, 0, 0, 0, 0]);

        let mut arr = [10, 5, 0, 2, 1];
        quick_sort(&mut arr, 0, 4);
        assert_eq!(arr, [0, 1, 2, 5, 10]);

        let mut arr = [10, 5, 0, 2, 1, 3, 4, 6, 9, 7, 8];
        quick_sort(&mut arr, 0, 10);
        assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        let mut arr = [10, 5, 0, 2, 1, 3, 7, 4, 6, 9, 7, 8];
        quick_sort(&mut arr, 0, 11);
        assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 7, 7, 8, 9, 10]);

        let mut arr = [10, 5, 0, 2, 1, 3, 7, 4, 6, 9, 7, 10, 8];
        quick_sort(&mut arr, 0, 12);
        assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 7, 7, 8, 9, 10, 10]);

        let mut arr = [10, 5, 0, 2, 1, 0, 3, 7, 4, 6, 9, 7, 10, 8];
        quick_sort(&mut arr, 0, 13);
        assert_eq!(arr, [0, 0, 1, 2, 3, 4, 5, 6, 7, 7, 8, 9, 10, 10]);

        let mut arr = [10, 5, 0, 2, 1, 0, 3, 7, 4, 6, 6, 9, 7, 6, 10, 8];
        quick_sort(&mut arr, 0, 15);
        assert_eq!(arr, [0, 0, 1, 2, 3, 4, 5, 6, 6, 6, 7, 7, 8, 9, 10, 10]);

        let mut vec = vec![0, 3, 3, 6, 8, 2];
        quick_sort(&mut vec, 0, 5);
        assert_eq!(vec, vec![0, 2, 3, 3, 6, 8]);
    }
}
