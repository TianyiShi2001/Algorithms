//! # Problem
//!
//! Reconstruct the original string from all substrings of length `l` of the string.
//! For example, reconstruct `"ATGCAGGT"` from `["ATG", "TGC", "GCA", "CAG", "AGG", "GGT"]`.
//! (note that the collection of *l*-mers is disordered)
//!
//! # Strategy
//!
//! This problem can be solved by finding a Eulerian path through the graph where vertices
//! represent `l`-mers and edges represent `l-1` overlaps among the `l`-mers. For example,
//! between two vertices `ATG` and `TGC`, there is a directed edge pointing from `ATG` to
//! `TGC`, because they have an `l-1`-mer` overlap, i.e. `"ATG"[1..] == "TGC"[.."TGC".len() - 1]`

use crate::algo::graph::eulerian_path::*;
use crate::algo::graph::UnweightedAdjacencyList;

pub fn reconstruct_string(lmers: &[&[u8]]) -> Result<Vec<u8>, EulerianPathError> {
    let l = lmers[0].len();
    let mut g = UnweightedAdjacencyList::with_size(lmers.len());
    let suffixes: Vec<_> = lmers.iter().map(|lmer| &lmer[1..]).collect();
    let prefixes: Vec<_> = lmers.iter().map(|lmer| &lmer[..l - 1]).collect();
    for (i, suffix) in suffixes.iter().enumerate() {
        for (j, prefix) in prefixes.iter().enumerate() {
            if suffix == prefix {
                g.add_directed_edge(i, j);
            }
        }
    }

    let path = g.eulerian_path()?;
    let mut res = lmers[path[0]].to_owned();
    for &i in &path[1..] {
        res.push(lmers[i][l - 1]);
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    fn _test(s: &[u8]) {
        let mut lmers: Vec<_> = s.windows(5).collect();
        let mut rng = thread_rng();
        lmers.shuffle(&mut rng);

        let reconstructed = reconstruct_string(&lmers).unwrap();

        assert_eq!(&reconstructed, &s);
    }

    #[test]
    fn test_reconstruct_string_from_lmers() {
        _test(b"The quick brown fox jumps over the lazy dog");
        _test(b"ATGGCGTGCA")
    }
}
