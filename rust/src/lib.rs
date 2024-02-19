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
}
