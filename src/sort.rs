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
    use crate::_test_utils::random_uniform_vec;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref V_I32: Vec<i32> = random_uniform_vec(-200, 200, 50);
        static ref V_I32_SORTED: Vec<i32> = {
            let mut v = V_I32.clone();
            v.sort_unstable();
            v
        };
        static ref V_USIZE: Vec<usize> = random_uniform_vec(0, 400, 50);
        static ref V_USIZE_SORTED: Vec<usize> = {
            let mut v = V_USIZE.clone();
            v.sort_unstable();
            v
        };
    }
    const V_I32_SINGLE: [i32; 1] = [5];

    fn test_sort_out_of_place<F: Fn(&[i32]) -> Vec<i32>>(f: F) {
        assert_eq!(&f(&*V_I32), &*V_I32_SORTED);
        assert_eq!(&f(&V_I32_SINGLE), &V_I32_SINGLE);
    }

    #[test]
    fn test_merge_sort() {
        test_sort_out_of_place(merge_sort::merge_sort);
    }

    #[test]
    fn test_bucket_sort() {
        test_sort_out_of_place(bucket_sort::bucket_sort);
    }

    fn test_sort_in_place<F: Fn(&mut [i32])>(f: F) {
        let mut w = V_I32.clone();
        f(&mut w);
        assert_eq!(&w, &*V_I32_SORTED);

        let mut single_item = V_I32_SINGLE.clone();
        f(&mut single_item);
        assert_eq!(&single_item, &V_I32_SINGLE);
    }

    #[test]
    fn test_selection_sort() {
        test_sort_in_place(selection_sort::selection_sort);
    }

    #[test]
    fn test_bubble_sort() {
        test_sort_in_place(bubble_sort::bubble_sort);
    }

    #[test]
    fn test_counting_sort() {
        test_sort_in_place(counting_sort::counting_sort);
    }

    #[test]
    fn test_insertion_sort() {
        test_sort_in_place(insertion_sort::insertion_sort);
    }

    #[test]
    fn test_heap_sort() {
        test_sort_in_place(heap_sort::heap_sort);
    }

    #[test]
    fn test_quick_sort() {
        test_sort_in_place(quick_sort::quick_sort);
    }

    #[test]
    fn test_radix_sort() {
        let sorted = radix_sort::radix_sort(&*V_USIZE);
        assert_eq!(&sorted, &*V_USIZE_SORTED);
    }
}
