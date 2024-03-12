pub fn linear_search_contains<T: PartialEq + Eq + PartialOrd + Ord + Copy>(arr: &[T], item: T) -> bool {
    for _item in arr {
        if item == *_item {
            return true;
        }
    }

    false
}

pub fn linear_search_index<T: PartialEq + Eq + PartialOrd + Ord + Copy>(arr: &[T], item: T) -> Option<usize> {
    let len = arr.len();
    for i in 0..len {
        if arr[i] == item {
            return Some(i)
        }
    }

    return None
}
