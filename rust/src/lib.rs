mod binary_search;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let arr: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let item = 10;
        assert_eq!(binary_search::binary_search(&arr, &item), true);
    }
}
