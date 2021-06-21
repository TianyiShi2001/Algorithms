pub mod bubble_sort;
pub mod counting_sort;
pub mod merge_sort;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sort_not_in_place() {
        let v = [10, 4, 6, 4, 8, -13, 2, 3];
        let expected = [-13, 2, 3, 4, 4, 6, 8, 10];
        let mut sorted;
        sorted = merge_sort::merge_sort(&v);
        assert_eq!(&sorted, &expected);
    }

    #[test]
    fn test_sort_in_place() {
        let v = [10, 4, 6, 4, 8, -13, 2, 3];
        let expected = [-13, 2, 3, 4, 4, 6, 8, 10];
        let mut w = v.clone();

        bubble_sort::bubble_sort(&mut w);
        assert_eq!(&w, &expected);

        let mut w = v.clone();
        counting_sort::counting_sort(&mut w);
        assert_eq!(&w, &expected);
    }
}
