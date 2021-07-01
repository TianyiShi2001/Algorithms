#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use algorithms_edu::{
        _test_utils::random_uniform_vec,
        string::suffix_array::{li2016::Huo2016, SuffixArray},
    };
    use test::{black_box, Bencher};

    fn bench_naive(b: &mut Bencher, sigma: u32, len: usize) {
        b.iter(|| {
            black_box({
                let mut s = random_uniform_vec(1, sigma, len);
                s.push(0);
                SuffixArray::from_str_very_naive(&s);
            });
        });
    }

    fn bench_li(b: &mut Bencher, sigma: u32, len: usize) {
        b.iter(|| {
            black_box({
                let mut s = random_uniform_vec(1, sigma, len);
                s.push(0);
                let mut sa = vec![0; s.len()];
                // let expected = SuffixArray::from_str_very_naive(&s).sa.clone();
                let mut solver = Huo2016::init(&mut s, &mut sa, Some(sigma as usize));
                solver.solve(true);
            });
        });
    }

    #[bench]
    fn bench_li_500_1000(b: &mut Bencher) {
        bench_li(b, 500, 1000);
    }
    #[bench]
    fn bench_li_500_10000(b: &mut Bencher) {
        bench_li(b, 500, 10000);
    }
    #[bench]
    fn bench_li_500_100000(b: &mut Bencher) {
        bench_li(b, 500, 100000);
    }
    #[bench]
    fn bench_li_500_1000000(b: &mut Bencher) {
        bench_li(b, 500, 1000000);
    }
    #[bench]
    fn bench_li_500_10000000(b: &mut Bencher) {
        bench_li(b, 500, 10000000);
    }

    #[bench]
    fn bench_naive_500_1000(b: &mut Bencher) {
        bench_naive(b, 500, 1000);
    }
    #[bench]
    fn bench_naive_500_10000(b: &mut Bencher) {
        bench_naive(b, 500, 10000);
    }
    #[bench]
    fn bench_naive_500_100000(b: &mut Bencher) {
        bench_naive(b, 500, 100000);
    }
    #[bench]
    fn bench_naive_500_1000000(b: &mut Bencher) {
        bench_naive(b, 500, 1000000);
    }
    #[bench]
    fn bench_naive_500_10000000(b: &mut Bencher) {
        bench_naive(b, 500, 10000000);
    }
}

// Sample result
// test tests::bench_li_500_1000      ... bench:     168,711 ns/iter (+/- 27,549)
// test tests::bench_li_500_10000     ... bench:   1,356,838 ns/iter (+/- 79,133)      x 8.0  x   8.0
// test tests::bench_li_500_100000    ... bench:  14,359,321 ns/iter (+/- 1,307,696)   x10.6  x  85.1
// test tests::bench_li_500_1000000   ... bench: 180,594,903 ns/iter (+/- 13,631,112)  x12.6  x1070.4
// test tests::bench_naive_500_1000    ... bench:      90,554 ns/iter (+/- 5,723)
// test tests::bench_naive_500_10000   ... bench:   1,262,911 ns/iter (+/- 58,702)      x13.9  x  10.3
// test tests::bench_naive_500_100000  ... bench:  16,251,237 ns/iter (+/- 661,166)     x12.9  x 179.5
// test tests::bench_naive_500_1000000 ... bench: 220,851,474 ns/iter (+/- 6,532,465)   x13.6  x2438.9
