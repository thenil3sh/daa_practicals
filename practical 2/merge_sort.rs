use std::fmt::Debug;

fn merge_sort<T: PartialOrd + Copy + Debug>(mut vec: Vec<T>) -> Vec<T> {
    let size = vec.len();
    if size <= 1 {
        return vec;
    }
    let right = vec.split_off(size / 2);
    let left = vec;
    merge(merge_sort(left), merge_sort(right))
}

fn merge<T: PartialOrd + Copy + Debug>(left: Vec<T>, right: Vec<T>) -> Vec<T> {
    let mut vec = vec![];
    let (mut i, mut j) = (0, 0);
    while i < left.len() && j < right.len() {
        vec.push(if left[i] < right[j] {
            i += 1;
            left[i - 1]
        } else {
            j += 1;
            right[j - 1]
        });
    }
    for iter in i..left.len() {
        vec.push(left[iter]);
    }

    for iter in j..right.len() {
        vec.push(right[iter]);
    }
    vec
}

#[cfg(test)]
mod merge_sort {
    use super::merge_sort;

    #[test]
    fn already_sorted_vector() {
        let vector = vec![1,2,3,4,5,6,7,8];
        assert_eq!(vec![1,2,3,4,5,6,7,8], merge_sort(vector));
    }

    #[test]
    fn average_unsorted_vector() {
        let vector = vec![23, 12, 45, 76, 32, 87, 99, 11];
        assert_eq!(vec![11, 12, 23, 32, 45, 76, 87, 99], merge_sort(vector));
    }

    #[test]
    fn vector_with_elements_sorted_in_opposite_order() {
        let vector = vec![10, 9, 8, 7, 6, 5, 4];
        assert_eq!(vec![4, 5, 6, 7, 8, 9, 10], merge_sort(vector));
    }
}