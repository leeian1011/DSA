#[allow(dead_code)]
pub fn binary_search<T: PartialEq + Eq + PartialOrd + Ord + Clone>(arr: &[T], item: &T) -> bool {
    let mut low = 0;
    let mut high = arr.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;
        let check = &arr[mid];
        if check == item {
            return true;
        }
        if &arr[mid] < item {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    false
}
