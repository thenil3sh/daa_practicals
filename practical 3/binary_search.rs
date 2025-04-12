fn binary_search(vec : &Vec<i32>, target : i32) -> bool {
    let mut upper_bound = vec.len();
    let mut lower_bound = 0;
    loop {
        let mid = (upper_bound + lower_bound)/2;

        if lower_bound >= upper_bound {
            return false;
        } else if vec[mid] == target {
            return true;
        } else if vec[mid] < target {
            lower_bound = mid + 1;
        } else if vec[mid] > target {
            upper_bound = mid - 1;
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn normal_search_if_target_exists_in_vec() {
        let vec = vec![1,22,32,43,44,66,77,100];
        assert!(binary_search(&vec, 22));
        assert!(binary_search(&vec, 32));
        assert!(binary_search(&vec, 100));
    }
    
    #[test]
    #[should_panic]
    fn search_if_target_doesnt_exist_in_vec() {
        let vec = vec![1,22,32,43,44,66,77,100];
        assert!(binary_search(&vec, 21));
        assert!(binary_search(&vec, 31));
        assert!(binary_search(&vec, 110));
    }


}