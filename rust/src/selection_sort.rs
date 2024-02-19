pub fn linear_selection_sort<T: PartialEq + PartialOrd + Eq + Ord + Copy>(arr: &mut [T]) {
    let length = arr.len();
    for i in 0..length {
        let mut smallest = arr[i];
        let mut smallest_index = i;
        for j in i..length {
            if smallest > arr[j] {
                smallest = arr[j];
                smallest_index = j;
            }
        }

        let temp_var = arr[i];
        arr[i] = smallest;
        arr[smallest_index] = temp_var;
    }
}
