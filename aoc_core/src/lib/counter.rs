use rustc_hash::FxHashMap;
use std::hash::Hash;

#[derive(Debug, Default)]
pub struct Counter<K> {
    data: FxHashMap<K, usize>,
}

impl<K> Counter<K>
where
    K: Eq + Hash,
{
    #[inline(always)]
    pub fn keys(&self) -> std::collections::hash_map::Keys<'_, K, usize> {
        self.data.keys()
    }

    #[inline(always)]
    pub fn values(&self) -> std::collections::hash_map::Values<'_, K, usize> {
        self.data.values()
    }

    #[inline(always)]
    pub fn increment(&mut self, key: K) {
        self.increment_by(key, 1);
    }

    pub fn increment_by(&mut self, key: K, value: usize) {
        self.data
            .entry(key)
            .and_modify(|c| *c += value)
            .or_insert(value);
    }
}

impl<K> FromIterator<K> for Counter<K>
where
    K: Default + Eq + Hash,
{
    fn from_iter<T: IntoIterator<Item = K>>(iter: T) -> Self {
        iter.into_iter().map(|elem| (elem, 1)).collect()
    }
}

impl<K> FromIterator<(K, usize)> for Counter<K>
where
    K: Default + Eq + Hash,
{
    fn from_iter<T: IntoIterator<Item = (K, usize)>>(iter: T) -> Self {
        iter.into_iter()
            .fold(Self::default(), |mut counter, (elem, count)| {
                counter.increment_by(elem, count);
                counter
            })
    }
}

impl<K> IntoIterator for Counter<K> {
    type Item = (K, usize);
    type IntoIter = std::collections::hash_map::IntoIter<K, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, K> IntoIterator for &'a Counter<K> {
    type Item = (&'a K, &'a usize);
    type IntoIter = std::collections::hash_map::Iter<'a, K, usize>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}
