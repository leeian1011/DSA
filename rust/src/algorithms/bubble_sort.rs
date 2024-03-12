pub fn bubble_sort<T: PartialEq + Eq + PartialOrd + Ord + Copy>(arr: &mut [T]) {
    let mut len = arr.len();

    loop {
        if len == 0 {
            break;
        }

        for i in 0..len {
            let j = i + 1;

            if j == len {
                break;
            }

            if arr[i] > arr[j] {
                let temp = arr[i];
                arr[i] = arr[j];
                arr[j] = temp;
            }
        }
        
        len -= 1;
    }
} 
