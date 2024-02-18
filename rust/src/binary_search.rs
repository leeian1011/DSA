pub fn binary_search<T: PartialEq + Eq + PartialOrd + Ord + Clone>(arr: &[T], item: &T) -> bool {
    let mut low = 0;
    let mut high = arr.len() - 1;

    loop {
        let mid = (low + high) / 2;
        if &arr[mid] == item {
            return true;
        }

        if mid == low {
            return false;
        }

        if &arr[mid] < item {
            low = mid;
        } else {
            high = mid;
        }
    }
}
