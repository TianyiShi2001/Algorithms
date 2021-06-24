use rand::{
    distributions::uniform::{SampleBorrow, SampleUniform, Uniform},
    thread_rng, Rng,
};
pub fn random_uniform_vec<T, X>(a: T, b: T, len: usize) -> Vec<X>
where
    X: SampleUniform,
    T: SampleBorrow<X> + Sized,
{
    thread_rng()
        .sample_iter(Uniform::new_inclusive(a, b))
        .take(len)
        .collect()
}

pub fn random_alphabetic_lowercase_string(len: usize) -> Vec<u8> {
    random_uniform_vec(0x61, 0x7A, len)
}
