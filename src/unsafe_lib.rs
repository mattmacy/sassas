use std::collections::HashMap;
use std::cell::RefCell;
use std::hash::Hash;
use std::ops::{Index, IndexMut};
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct MutMap<K: Eq + Hash, V: Default> {
    map: HashMap<K, RefCell<V>>,
}

impl<K: Eq + Hash, V: Default> MutMap<K, V> {
    pub fn new() -> Self {
        MutMap { map: HashMap::new() }
    }
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        let result = self.map.insert(k, RefCell::new(v));
        match result {
            None => None,
            Some(v) => Some(v.into_inner()),
        }
    }
}

impl<K: Hash + Eq + Clone + Debug, V: Default> Index<K> for MutMap<K, V> {
    type Output = V;
    fn index(&self, idx: K) -> &Self::Output {
        let map = &self.map;
        if !map.contains_key(&idx) {
            panic!("{:?} not found", idx)
        }
        let cntp = map[&idx].as_ptr();
        unsafe { &*cntp }
    }
}
impl<K: Hash + Eq + Clone + Debug, V: Default> IndexMut<K> for MutMap<K, V> {
    fn index_mut(&mut self, idx: K) -> &mut Self::Output {
        let map = &mut self.map;
        if !map.contains_key(&idx) {
            map.insert(idx.clone(), RefCell::new(V::default()));
        }
        let cntp = map[&idx].as_ptr();
        unsafe { &mut *cntp }
    }
}
