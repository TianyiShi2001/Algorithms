/// An implementation of Radix Sort.
///
/// See https://en.wikipedia.org/wiki/Radix_sort for details on runtime and complexity Radix sorts
/// operates in O(nw) time, where n is the number of keys, and w is the key length where w is
/// constant on primitive types like Integer which gives it a better performance than other
/// compare-based sort algorithms, like i.e. QuickSort
///
/// - Time Complexity: O(nw)

// TODO: simplify? support negative integers?

pub fn radix_sort(v: &[usize]) -> Vec<usize> {
    if v.len() <= 1 {
        v.to_owned()
    } else {
        let mx = *v.iter().max().unwrap();
        let mut ndigits = number_of_digits(mx);
        let mut place = 1;
        let mut a = v.to_owned();
        let mut b = vec![0; v.len()];
        let mut i = 0;
        while ndigits > 0 {
            if i % 2 == 0 {
                counting_sort(&mut a, place, &mut b);
            } else {
                counting_sort(&mut b, place, &mut a);
            }
            ndigits -= 1;
            place *= 10;
            i += 1;
        }
        if i % 2 == 0 {
            a
        } else {
            b
        }
    }
}

fn number_of_digits(n: usize) -> usize {
    (n as f64).log10() as usize + 1
}

fn counting_sort<'a>(v: &'a mut [usize], place: usize, sorted: &'a mut [usize]) {
    const RANGE: usize = 10;
    let mut frequency = vec![0; RANGE];
    let digit = v.iter().map(|n| (*n / place) % RANGE).collect::<Vec<_>>();
    for d in &digit {
        frequency[*d] += 1;
    }

    for i in 1..RANGE {
        // now `frequency[i]` actually represents the index in the
        // sorted slice, of the next value with `i` at the relevant place
        frequency[i] += frequency[i - 1];
    }
    for (&n, &d) in v.iter().zip(digit.iter()).rev() {
        sorted[frequency[d] - 1] = n;
        frequency[d] -= 1;
    }
}
