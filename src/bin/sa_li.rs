use algorithms_edu::{_test_utils::random_uniform_vec, string::suffix_array::li2016::Huo2016};

fn main() {
    solve(500, 300_000_000);
}

fn solve(sigma: u32, len: usize) {
    let mut s = random_uniform_vec(1, sigma, len);
    s.push(0);
    let mut sa = vec![0; s.len()];
    let mut solver = Huo2016::init(&mut s, &mut sa, Some(sigma as usize));
    solver.solve(true);
    assert!(!sa.is_empty());
}
