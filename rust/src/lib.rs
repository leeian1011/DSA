mod algorithms;
mod data_structures;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let arr: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let item = 9;
        assert_eq!(algorithms::binary_search::binary_search(&arr, &item), true);
    }

    #[test]
    fn it_works_ssort() {
        let mut arr: [i32; 5] = [5, 3, 2, 4, 1];
        algorithms::selection_sort::linear_selection_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn it_works_recursive_sum() {
        let arr: [i32; 0] = [];
        let arr2: [i32; 5] = [1, 2, 3, 4, 5];
        let arr3: [i32; 2] = [10, 5];
        assert_eq!(algorithms::recursive::recursive_sum(&arr), 0);
        assert_eq!(algorithms::recursive::recursive_sum(&arr2), 15);
        assert_eq!(algorithms::recursive::recursive_count(&arr2), 5);
        assert_eq!(algorithms::recursive::recursive_max(&arr3), 10);
    }

    #[test]
    fn it_works_qsort() {
        let mut arr: [u8; 10] = [7, 3, 1, 2, 8, 9, 4, 0, 5, 6];
        let mut arr2: [u8; 10] = [7, 7, 2, 2, 18, 4, 3, 1, 222, 6];
        algorithms::quick_sort::qsort(&mut arr);
        algorithms::quick_sort::qsort(&mut arr2);
        assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(arr2, [1, 2, 2, 3, 4, 6, 7, 7, 18, 222]);
    }

    #[test]
    fn it_works_linear_search() {
        let arr: [u8; 10] = [0, 3, 4, 8, 9, 22, 37, 11, 3, 14];
        assert_eq!(algorithms::linear_search::linear_search_index(&arr, 17), None);
        assert_eq!(algorithms::linear_search::linear_search_index(&arr, 3), Some(1));
        assert_eq!(algorithms::linear_search::linear_search_contains(&arr, 255), false);
        assert_eq!(algorithms::linear_search::linear_search_contains(&arr, 22), true);
    }

    #[test]
    fn it_works_bubble_sort() {
        let mut arr: [u8; 10] = [3, 5, 10, 124, 255, 20, 73, 64, 98, 100];
        algorithms::bubble_sort::bubble_sort(&mut arr);
        assert_eq!(arr, [3, 5, 10, 20, 64, 73, 98, 100, 124, 255]);
    }
}
