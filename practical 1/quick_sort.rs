use std::fmt::Debug;

fn pivot<T : PartialOrd + Debug>(vector: &mut [T]) -> usize {
    let mut pivot = 0;
    for i in 1..vector.len() {
        if vector[i] < vector[pivot] {
            vector.swap(pivot + 1, i);
            vector.swap(pivot, pivot + 1);
            pivot += 1;
        }
    }
    pivot
}

pub fn quick_sort<T : PartialOrd + Debug>(vector: &mut [T]) {
    if vector.len() <= 1 {
        return;
    }
    let pivot = pivot::<T>(vector);
    let (left, right) = vector.split_at_mut(pivot);
    println!("Left : {left:?} Right : {right:?}");
    quick_sort(left);
    quick_sort(&mut right[1..]);
}

#[cfg(test)]
mod quick_sort {
    use super::quick_sort;

    #[test] 
    fn any_normal_vector() {
        let mut vector = vec![2, 3, 12, 3, 2, 66, 87, 32, 9];
        quick_sort(&mut vector);
        assert_eq!(vec![2, 2, 3, 3, 9, 12, 32, 66, 87], vector);
    }

    #[test]
    fn vector_sorted_in_opposite_order() {
        let mut vector = vec![8, 7, 6, 5, 5, 4, 3, 2, 1];
        quick_sort(&mut vector);
        assert_eq!(vector, vec![1, 2, 3, 4, 5, 5, 6, 7, 8]);
    }

    #[test]
    fn already_sorted_vector() {
        let mut vector = vec![1,2,3,4,5,6,7,8];
        quick_sort(&mut vector);
        assert_eq!(vector, vec![1,2,3,4,5,6,7,8]);
    }
}