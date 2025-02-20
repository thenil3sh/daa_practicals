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

fn quick_sort<T : PartialOrd + Debug>(vector: &mut [T]) {
    if vector.len() <= 1 {
        return;
    }
    let pivot = pivot::<T>(vector);
    let (left, right) = vector.split_at_mut(pivot);
    println!("Left : {left:?} Right : {right:?}");
    quick_sort(left);
    quick_sort(&mut right[1..]);
}

fn main() {
    let mut vector = vec![5, 7, 1, 6, 8, 6, 11, 0];
    println!("Vector before sort : {vector:?}");
    quick_sort(&mut vector);
    println!("{vector:?}");
}
