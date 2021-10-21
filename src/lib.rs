use ahash::AHasher;
use rayon::prelude::*;
use std::{
    collections::{BTreeSet, HashSet},
    hash::Hasher,
};

type Entry = Vec<u8>;
type EntryList = Vec<Entry>;
type Hash = u64;

pub fn eq_by_sorting_seq(mut x: EntryList, mut y: EntryList) -> bool {
    x.sort_unstable();
    y.sort_unstable();
    x == y
}

pub fn eq_by_sorting_par(mut x: EntryList, mut y: EntryList) -> bool {
    x.par_sort_unstable();
    y.par_sort_unstable();
    x == y
}

pub fn hash_seq(x: EntryList) -> Hash {
    // Hash individual entries
    let sorted_hashes = x
        .into_iter()
        .map(|e| {
            let mut hasher = AHasher::default();
            hasher.write(&e[..]);
            hasher.finish()
        })
        .collect::<BTreeSet<_>>();

    // Hash the sorted hash list
    sorted_hashes
        .into_iter()
        .fold(AHasher::default(), |mut hasher, elem| {
            hasher.write_u64(elem);
            hasher
        })
        .finish()
}

pub fn hash_par(x: EntryList) -> Hash {
    // Same as above, but parallel
    let sorted_hashes = x
        .into_par_iter()
        .map(|e| {
            let mut hasher = AHasher::default();
            hasher.write(&e[..]);
            hasher.finish()
        })
        .collect::<BTreeSet<_>>();

    // ...however, the final hashing must be sequential, and that's sad
    sorted_hashes
        .into_iter()
        .fold(AHasher::default(), |mut hasher, elem| {
            hasher.write_u64(elem);
            hasher
        })
        .finish()
}

// If we know that we want to compare for equality, we can do it...
pub fn eq_by_hashing_seq(x: EntryList, y: EntryList) -> bool {
    let make_hash_set = |list: EntryList| {
        list.into_iter()
            .map(|e| {
                let mut hasher = AHasher::default();
                hasher.write(&e[..]);
                hasher.finish()
            })
            .collect::<HashSet<_>>()
    };
    make_hash_set(x) == make_hash_set(y)
}

// ...and it will parallelize a bit better
pub fn eq_by_hashing_par(x: EntryList, y: EntryList) -> bool {
    let make_hash_set = |list: EntryList| {
        list.into_par_iter()
            .map(|e| {
                let mut hasher = AHasher::default();
                hasher.write(&e[..]);
                hasher.finish()
            })
            .collect::<HashSet<_>>()
    };
    make_hash_set(x) == make_hash_set(y)
}

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
    fn same_eq_by_sorting_seq(data: EntryList) {
        same_eq(data, eq_by_sorting_seq);
    }

    #[quickcheck]
    fn same_eq_by_sorting_par(data: EntryList) {
        same_eq(data, eq_by_sorting_par);
    }

    #[quickcheck]
    fn same_eq_by_hashing_seq(data: EntryList) {
        same_eq(data, eq_by_hashing_seq);
    }

    #[quickcheck]
    fn same_eq_by_hashing_par(data: EntryList) {
        same_eq(data, eq_by_hashing_par);
    }

    fn same_hash(data: EntryList, mut hash: impl FnMut(EntryList) -> Hash) {
        same_eq(data, |x, y| hash(x) == hash(y))
    }

    #[quickcheck]
    fn same_hash_seq(data: EntryList) {
        same_hash(data, hash_seq);
    }

    #[quickcheck]
    fn same_hash_par(data: EntryList) {
        same_hash(data, hash_par);
    }

    fn pair_eq(x: EntryList, y: EntryList, tested_eq: impl FnOnce(EntryList, EntryList) -> bool) {
        assert_eq!(eq_by_sorting_seq(x.clone(), y.clone()), tested_eq(x, y));
    }

    #[quickcheck]
    fn pair_eq_sorting_par(x: EntryList, y: EntryList) {
        pair_eq(x, y, eq_by_sorting_par)
    }

    #[quickcheck]
    fn pair_eq_hashing_seq(x: EntryList, y: EntryList) {
        pair_eq(x, y, eq_by_hashing_seq)
    }

    #[quickcheck]
    fn pair_eq_hashing_par(x: EntryList, y: EntryList) {
        pair_eq(x, y, eq_by_hashing_par)
    }

    fn pair_hash(x: EntryList, y: EntryList, mut tested_hash: impl FnMut(EntryList) -> Hash) {
        assert_eq!(
            eq_by_sorting_seq(x.clone(), y.clone()),
            tested_hash(x) == tested_hash(y)
        );
    }

    #[quickcheck]
    fn pair_hash_seq(x: EntryList, y: EntryList) {
        pair_hash(x, y, hash_seq)
    }

    #[quickcheck]
    fn pair_hash_par(x: EntryList, y: EntryList) {
        pair_hash(x, y, hash_par)
    }
}
