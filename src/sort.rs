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
    use lazy_static::lazy_static;
    use rand::{distributions::Uniform, thread_rng, Rng};

    lazy_static! {
        static ref V_i32: Vec<i32> = {
            let mut rng = thread_rng();
            rng.sample_iter(Uniform::new_inclusive(-200, 200))
                .take(50)
                .collect()
        };
        static ref V_i32_SORTED: Vec<i32> = {
            let mut v = V_i32.clone();
            v.sort_unstable();
            v
        };
    }

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

    fn test_sort_in_place<F: Fn(&mut [i32])>(f: F) {
        let mut w = V_i32.clone();
        f(&mut w);
        assert_eq!(&w, &*V_i32_SORTED);
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
}
