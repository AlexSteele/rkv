use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeMap;
use std::hash::Hash;
use std::hash::Hasher;
use std::iter::Iterator;
use std::ops::Bound;

// A consistent hash ring inspired by Karger et al.'s
// https://www.akamai.com/us/en/multimedia/documents/technical-publication/consistent-hashing-and-random-trees-distributed-caching-protocols-for-relieving-hot-spots-on-the-world-wide-web-technical-publication.pdf
pub struct HashRing<T> {
    entries: BTreeMap<u64, T>,
    replicas: i32,
}

// TODO: Replace DefaultHasher. Stability not guaranteed across rust versions.
impl<T: Hash + Clone + Eq> HashRing<T> {
    pub fn new(replicas: i32) -> Self {
        assert!(replicas > 0, "replicas must be > 0");
        Self {
            entries: BTreeMap::new(),
            replicas: replicas,
        }
    }

    pub fn insert(&mut self, bucket: T) {
        for point in self.points(&bucket) {
            // Overwrites the prior entry on collision
            self.entries.insert(point, bucket.clone());
        }
    }

    pub fn remove(&mut self, bucket: &T) {
        for point in self.points(bucket) {
            // Removes the prior entry on collision
            self.entries.remove(&point);
        }
    }

    pub fn get<V: Hash>(&self, item: &V) -> Option<&T> {
        self.successors(item).next()
    }

    pub fn successors<V: Hash>(&self, item: &V) -> impl Iterator<Item = &T> {
        let point = self.point(item);
        let r1 = self
            .entries
            .range((Bound::Included(point), Bound::Unbounded));
        let r2 = self
            .entries
            .range((Bound::Included(0 as u64), Bound::Excluded(point)));

        r1.chain(r2).map(|(_, v)| v)
    }

    fn point<V: Hash>(&self, v: &V) -> u64 {
        let mut h = DefaultHasher::new();
        v.hash(&mut h);
        h.finish()
    }

    fn points(&self, bucket: &T) -> Vec<u64> {
        let mut res = Vec::new();
        for replica in 0..self.replicas {
            let mut h = DefaultHasher::new();
            bucket.hash(&mut h);
            replica.hash(&mut h);
            res.push(h.finish());
        }
        res
    }
}

// TODO: Test balance guarantees
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_ring() {
        let replicas = 8;
        let mut r: HashRing<i32> = HashRing::new(replicas);

        // empty
        assert_eq!(r.get(&0), None);

        // single bucket
        r.insert(0);
        assert_eq!(r.get(&42), Some(&0));

        let successors: Vec<_> = r.successors(&99).map(|v| *v).collect();
        let expected: Vec<_> = std::iter::repeat(0).take(replicas as usize).collect();
        assert_eq!(successors, expected);

        // empty
        r.remove(&0);
        assert_eq!(r.get(&0), None);

        // N buckets
        let N = 4;
        let buckets: Vec<_> = (0..N).collect();
        for bucket in &buckets {
            r.insert(*bucket);
        }
        assert_eq!(buckets.contains(r.get(&"foo").unwrap()), true);
        let successors: Vec<_> = r.successors(&"bar").cloned().collect();
        assert_eq!(successors.len(), buckets.len() * replicas as usize);
    }
}
