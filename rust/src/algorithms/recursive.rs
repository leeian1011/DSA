#[allow(dead_code)]
pub fn recursive_sum(arr: &[i32]) -> i32 {
    let len = arr.len();

    if len == 0 {
        return 0;
    }

    if len == 1 {
        return arr[0];
    }

    arr[0] + recursive_sum(&arr[1..len])
}

#[allow(dead_code)]
pub fn recursive_count(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        return 0;
    }

    1 + recursive_count(&arr[1..])
}

#[allow(dead_code)]
pub fn recursive_max(arr: &[i32]) -> i32 {
    if arr.len() == 2 {
        match arr[0].cmp(&arr[1]) {
            std::cmp::Ordering::Less => return arr[1],
            std::cmp::Ordering::Greater => return arr[0],
            std::cmp::Ordering::Equal => return arr[0],
        }
    }

    let max = recursive_max(&arr[1..]);

    if arr[0] > max {
        arr[0]
    } else {
        max
    }
}
