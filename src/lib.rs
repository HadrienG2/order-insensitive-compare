use ahash::AHasher;
use rayon::prelude::*;
use sha2::{digest::Output, Digest, Sha256};
use std::hash::Hasher;

type Entry = Vec<u8>;
type EntryList = Vec<Entry>;

fn par_eq<T: Eq + Sync>(x: Vec<T>, y: Vec<T>) -> bool {
    let chunk_size = x.len() / rayon::current_num_threads();
    x.len() == y.len()
        && x.par_chunks(chunk_size)
            .zip(y.par_chunks(chunk_size))
            .all(|(xe, ye)| xe == ye)
}

// ===

pub fn eq_by_sorting_seq(mut x: EntryList, mut y: EntryList) -> bool {
    x.sort_unstable();
    y.sort_unstable();
    x == y
}

pub fn eq_by_sorting_par(mut x: EntryList, mut y: EntryList) -> bool {
    x.par_sort_unstable();
    y.par_sort_unstable();
    par_eq(x, y)
}

// ===

pub fn ahash_seq(x: EntryList) -> u64 {
    // Hash individual entries
    let mut hashes = x
        .into_iter()
        .map(|e| {
            let mut hasher = AHasher::default();
            hasher.write(&e[..]);
            hasher.finish()
        })
        .collect::<Vec<_>>();

    // Sort the hashes
    hashes.sort_unstable();

    // Hash the sorted hash list
    hashes
        .into_iter()
        .fold(AHasher::default(), |mut hasher, elem| {
            hasher.write_u64(elem);
            hasher
        })
        .finish()
}

pub fn ahash_par(x: EntryList) -> u64 {
    // Same as above, but parallel
    let mut hashes = x
        .into_par_iter()
        .map(|e| {
            let mut hasher = AHasher::default();
            hasher.write(&e[..]);
            hasher.finish()
        })
        .collect::<Vec<_>>();
    hashes.par_sort_unstable();

    // ...however, the final hashing must be sequential, and that's sad
    hashes
        .into_iter()
        .fold(AHasher::default(), |mut hasher, elem| {
            hasher.write_u64(elem);
            hasher
        })
        .finish()
}

// ---

// If we know that we want to compare for equality, we can do it...
pub fn eq_by_ahash_seq(x: EntryList, y: EntryList) -> bool {
    let sorted_hashes = |list: EntryList| {
        let mut hashes = list
            .into_iter()
            .map(|e| {
                let mut hasher = AHasher::default();
                hasher.write(&e[..]);
                hasher.finish()
            })
            .collect::<Vec<_>>();
        hashes.sort_unstable();
        hashes
    };
    sorted_hashes(x) == sorted_hashes(y)
}

// ...and then there is no hashing at the end, only a comparison, which is
// faster (same memory traffic, no hashing overhead) and parallelizable.
pub fn eq_by_ahash_par(x: EntryList, y: EntryList) -> bool {
    let sorted_hashes = |list: EntryList| {
        let mut hashes = list
            .into_par_iter()
            .map(|e| {
                let mut hasher = AHasher::default();
                hasher.write(&e[..]);
                hasher.finish()
            })
            .collect::<Vec<_>>();
        hashes.par_sort_unstable();
        hashes
    };
    sorted_hashes(x) == sorted_hashes(y) // par_eq tested, but not beneficial for 1k hashes
}

// ===

pub fn sha256_seq(x: EntryList) -> Output<Sha256> {
    // Hash individual entries
    let mut hashes = x
        .into_iter()
        .map(|e| Sha256::digest(&e[..]))
        .collect::<Vec<_>>();

    // Sort the hashes
    hashes.sort_unstable();

    // Hash the sorted hash list
    hashes
        .into_iter()
        .fold(Sha256::new(), |hasher, elem| hasher.chain(elem.as_slice()))
        .finalize()
}

pub fn sha256_par(x: EntryList) -> Output<Sha256> {
    // Same as above, but parallel
    let mut hashes = x
        .into_par_iter()
        .map(|e| Sha256::digest(&e[..]))
        .collect::<Vec<_>>();
    hashes.par_sort_unstable();

    // ...however, the final hashing must be sequential, and that's sad
    hashes
        .into_iter()
        .fold(Sha256::new(), |hasher, elem| hasher.chain(elem.as_slice()))
        .finalize()
}

// ---

// If we know that we want to compare for equality, we can do it...
pub fn eq_by_sha256_seq(x: EntryList, y: EntryList) -> bool {
    let sorted_hashes = |list: EntryList| {
        let mut hashes = list
            .into_iter()
            .map(|e| Sha256::digest(&e[..]))
            .collect::<Vec<_>>();
        hashes.sort_unstable();
        hashes
    };
    sorted_hashes(x) == sorted_hashes(y)
}

// ...and then there is no hashing at the end, only a comparison, which is
// faster (same memory traffic, no hashing overhead) and parallelizable.
pub fn eq_by_sha256_par(x: EntryList, y: EntryList) -> bool {
    let sorted_hashes = |list: EntryList| {
        let mut hashes = list
            .into_par_iter()
            .map(|e| Sha256::digest(&e[..]))
            .collect::<Vec<_>>();
        hashes.par_sort_unstable();
        hashes
    };
    sorted_hashes(x) == sorted_hashes(y) // par_eq tested, but not beneficial for 1k hashes
}

// ===

pub fn blake3_seq(x: EntryList) -> blake3::Hash {
    // Hash individual entries
    let mut hashes = x
        .into_iter()
        .map(|e| blake3::hash(&e[..]))
        .collect::<Vec<_>>();

    // Sort the hashes
    hashes.sort_unstable_by_key(|hash| *hash.as_bytes());

    // Hash the sorted hash list
    hashes
        .into_iter()
        .fold(blake3::Hasher::new(), |mut hasher, elem| {
            hasher.update(elem.as_bytes());
            hasher
        })
        .finalize()
}

pub fn blake3_par(x: EntryList) -> blake3::Hash {
    // Same as above, but parallel
    let mut hashes = x
        .into_par_iter()
        .map(|e| blake3::hash(&e[..]))
        .collect::<Vec<_>>();
    hashes.par_sort_unstable_by_key(|hash| *hash.as_bytes());

    // ...however, the final hashing must be sequential, and that's sad
    hashes
        .into_iter()
        .fold(blake3::Hasher::new(), |mut hasher, elem| {
            hasher.update(elem.as_bytes());
            hasher
        })
        .finalize()
}

// ---

// If we know that we want to compare for equality, we can do it...
pub fn eq_by_blake3_seq(x: EntryList, y: EntryList) -> bool {
    let sorted_hashes = |list: EntryList| {
        let mut hashes = list
            .into_iter()
            .map(|e| blake3::hash(&e[..]))
            .collect::<Vec<_>>();
        hashes.sort_unstable_by_key(|hash| *hash.as_bytes());
        hashes
    };
    sorted_hashes(x) == sorted_hashes(y)
}

// ...and then there is no hashing at the end, only a comparison, which is
// faster (same memory traffic, no hashing overhead) and parallelizable.
pub fn eq_by_blake3_par(x: EntryList, y: EntryList) -> bool {
    let sorted_hashes = |list: EntryList| {
        let mut hashes = list
            .into_par_iter()
            .map(|e| blake3::hash(&e[..]))
            .collect::<Vec<_>>();
        hashes.par_sort_unstable_by_key(|hash| *hash.as_bytes());
        hashes
    };
    sorted_hashes(x) == sorted_hashes(y) // par_eq tested, but not beneficial for 1k hashes
}

// ===

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;
    use rand::prelude::*;

    fn same_eq(data: EntryList, eq: impl FnOnce(EntryList, EntryList) -> bool) {
        let mut rng = rand::thread_rng();
        let mut shuffled = data.clone();
        shuffled.shuffle(&mut rng);
        assert!(eq(data.clone(), shuffled.clone()));
    }

    #[quickcheck]
    fn same_eq_sorting_seq(data: EntryList) {
        same_eq(data, eq_by_sorting_seq);
    }

    #[quickcheck]
    fn same_eq_sorting_par(data: EntryList) {
        same_eq(data, eq_by_sorting_par);
    }

    #[quickcheck]
    fn same_eq_ahash_seq(data: EntryList) {
        same_eq(data, eq_by_ahash_seq);
    }

    #[quickcheck]
    fn same_eq_ahash_par(data: EntryList) {
        same_eq(data, eq_by_ahash_par);
    }

    #[quickcheck]
    fn same_eq_sha256_seq(data: EntryList) {
        same_eq(data, eq_by_sha256_seq);
    }

    #[quickcheck]
    fn same_eq_sha256_par(data: EntryList) {
        same_eq(data, eq_by_sha256_par);
    }

    #[quickcheck]
    fn same_eq_blake3_seq(data: EntryList) {
        same_eq(data, eq_by_blake3_seq);
    }

    #[quickcheck]
    fn same_eq_blake3_par(data: EntryList) {
        same_eq(data, eq_by_blake3_par);
    }

    fn same_hash<O: Eq>(data: EntryList, mut hash: impl FnMut(EntryList) -> O) {
        same_eq(data, |x, y| hash(x) == hash(y))
    }

    #[quickcheck]
    fn same_ahash_seq(data: EntryList) {
        same_hash(data, ahash_seq);
    }

    #[quickcheck]
    fn same_ahash_par(data: EntryList) {
        same_hash(data, ahash_par);
    }

    #[quickcheck]
    fn same_sha256_seq(data: EntryList) {
        same_hash(data, sha256_seq);
    }

    #[quickcheck]
    fn same_sha256_par(data: EntryList) {
        same_hash(data, sha256_par);
    }

    #[quickcheck]
    fn same_blake3_seq(data: EntryList) {
        same_hash(data, blake3_seq);
    }

    #[quickcheck]
    fn same_blake3_par(data: EntryList) {
        same_hash(data, blake3_par);
    }

    fn pair_eq(x: EntryList, y: EntryList, tested_eq: impl FnOnce(EntryList, EntryList) -> bool) {
        assert_eq!(eq_by_sorting_seq(x.clone(), y.clone()), tested_eq(x, y));
    }

    #[quickcheck]
    fn pair_eq_sorting_par(x: EntryList, y: EntryList) {
        pair_eq(x, y, eq_by_sorting_par)
    }

    #[quickcheck]
    fn pair_eq_ahash_seq(x: EntryList, y: EntryList) {
        pair_eq(x, y, eq_by_ahash_seq)
    }

    #[quickcheck]
    fn pair_eq_ahash_par(x: EntryList, y: EntryList) {
        pair_eq(x, y, eq_by_ahash_par)
    }

    #[quickcheck]
    fn pair_eq_sha256_seq(x: EntryList, y: EntryList) {
        pair_eq(x, y, eq_by_sha256_seq)
    }

    #[quickcheck]
    fn pair_eq_sha256_par(x: EntryList, y: EntryList) {
        pair_eq(x, y, eq_by_sha256_par)
    }

    #[quickcheck]
    fn pair_eq_blake3_seq(x: EntryList, y: EntryList) {
        pair_eq(x, y, eq_by_blake3_seq)
    }

    #[quickcheck]
    fn pair_eq_blake3_par(x: EntryList, y: EntryList) {
        pair_eq(x, y, eq_by_blake3_par)
    }

    fn pair_hash<O: Eq>(x: EntryList, y: EntryList, mut tested_hash: impl FnMut(EntryList) -> O) {
        assert_eq!(
            eq_by_sorting_seq(x.clone(), y.clone()),
            tested_hash(x) == tested_hash(y)
        );
    }

    #[quickcheck]
    fn pair_ahash_seq(x: EntryList, y: EntryList) {
        pair_hash(x, y, ahash_seq)
    }

    #[quickcheck]
    fn pair_ahash_par(x: EntryList, y: EntryList) {
        pair_hash(x, y, ahash_par)
    }

    #[quickcheck]
    fn pair_sha256_seq(x: EntryList, y: EntryList) {
        pair_hash(x, y, sha256_seq)
    }

    #[quickcheck]
    fn pair_sha256_par(x: EntryList, y: EntryList) {
        pair_hash(x, y, sha256_par)
    }

    #[quickcheck]
    fn pair_blake3_seq(x: EntryList, y: EntryList) {
        pair_hash(x, y, blake3_seq)
    }

    #[quickcheck]
    fn pair_blake3_par(x: EntryList, y: EntryList) {
        pair_hash(x, y, blake3_par)
    }
}
