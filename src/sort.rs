pub mod bubble_sort;
pub mod bucket_sort;
pub mod counting_sort;
pub mod heap_sort;
pub mod insertion_sort;
pub mod merge_sort;
pub mod quick_sort;
pub mod radix_sort;
pub mod selection_sort;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_out_of_placee() {
        let v = [10, 4, 6, 4, 8, -13, 2, 3];
        let expected = [-13, 2, 3, 4, 4, 6, 8, 10];
        let mut sorted;
        sorted = merge_sort::merge_sort(&v);
        assert_eq!(&sorted, &expected);
        sorted = bucket_sort::bucket_sort(&v);
        assert_eq!(&sorted, &expected);
    }

    #[test]
    fn test_radix_sort() {
        let v = [387, 468, 134, 123, 68, 221, 769, 37, 7, 890, 1, 587];
        let expected = [1, 7, 37, 68, 123, 134, 221, 387, 468, 587, 769, 890];
        let sorted = radix_sort::radix_sort(&v);
        assert_eq!(&sorted, &expected);
    }

    #[test]
    fn test_sort_in_place() {
        let v = [10, 4, 6, 4, 8, -13, 2, 3];
        let expected = [-13, 2, 3, 4, 4, 6, 8, 10];

        let mut w = v.clone();
        selection_sort::selection_sort(&mut w);
        assert_eq!(&w, &expected);

        let mut w = v.clone();
        bubble_sort::bubble_sort(&mut w);
        assert_eq!(&w, &expected);

        let mut w = v.clone();
        counting_sort::counting_sort(&mut w);
        assert_eq!(&w, &expected);

        let mut w = v.clone();
        insertion_sort::insertion_sort(&mut w);
        assert_eq!(&w, &expected);

        let mut w = v.clone();
        heap_sort::heap_sort(&mut w);
        assert_eq!(&w, &expected);

        let mut w = v.clone();
        quick_sort::quick_sort(&mut w);
        assert_eq!(&w, &expected);
    }
}
