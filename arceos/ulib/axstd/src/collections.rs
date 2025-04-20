#[cfg(feature = "alloc")]
#[doc(no_inline)]
pub use alloc::collections;

use collections::btree_map::{BTreeMap, Iter};

pub struct HashMap<K, V> {
    hashmap: BTreeMap<K, V>,
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        Self {
            hashmap: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V>
    where
        K: Ord,
    {
        self.hashmap.insert(k, v)
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        self.hashmap.iter()
    }
}
