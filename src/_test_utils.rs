use rand::{
    distributions::uniform::{SampleBorrow, SampleUniform, Uniform},
    thread_rng, Rng,
};
pub(crate) fn random_uniform_vec<T: SampleBorrow<X> + Sized, X: SampleUniform>(
    a: T,
    b: T,
) -> Vec<X> {
    thread_rng()
        .sample_iter(Uniform::new_inclusive(a, b))
        .take(50)
        .collect()
}
