// This qsort takes it's pivot from the end
#[allow(dead_code)]
pub fn qsort(arr: &mut [u8]) {
    let length = arr.len();
    if length < 2 {
        return;
    }

    if length == 2 {
        if arr[0] > arr[1] {
            let temp = arr[0];
            arr[0] = arr[1];
            arr[1] = temp;
        }

        return;
    }

    let pivot = arr[length - 1];
    let mut actual_index = 0;

    for i in 0..(length - 1) {
        if arr[i] <= pivot {
            let temp = arr[i];
            arr[i] = arr[actual_index];
            arr[actual_index] = temp;
            actual_index += 1;
        }
    }

    let temp = arr[actual_index];
    arr[actual_index] = arr[length - 1];
    arr[length - 1] = temp;

    qsort(&mut arr[..actual_index]);
    qsort(&mut arr[actual_index..]);
}
