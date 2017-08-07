use std::collections::HashMap;
use std::cell::RefCell;
use std::hash::Hash;
use std::collections::hash_map::Iter;
use std::ops::{Index, IndexMut};
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct MutMap<K: Eq + Hash, V: Default> {
    map: HashMap<K, RefCell<V>>,
}
pub struct MutMapIter<'a, K: 'a, V: 'a + Default> {
    iter: Iter<'a, K, RefCell<V>>,
}
impl<'a, K, V: Default + Clone> Iterator for MutMapIter<'a, K, V> {
    type Item = (&'a K, V);
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => None,
            Some((k, rv)) => {
                let v = rv.borrow().clone();
                Some((k, v))
            }
        }
    }
}
impl<K: Eq + Hash, V: Default> Default for MutMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Eq + Hash, V: Default> MutMap<K, V> {
    pub fn new() -> Self {
        MutMap { map: HashMap::new() }
    }
    pub fn iter(&self) -> MutMapIter<K, V> {
        MutMapIter { iter: self.map.iter() }
    }
    pub fn len(&self) -> usize {
        self.map.len()
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

#[derive(Clone, Debug)]
pub struct MutStrMap<V: Default> {
    map: MutMap<String, V>,
}

impl<V: Default> Default for MutStrMap<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V: Default> MutStrMap<V> {
    pub fn new() -> Self {
        MutStrMap { map: MutMap::new() }
    }
    pub fn iter(&self) -> MutMapIter<String, V> {
        self.map.iter()
    }
    pub fn len(&self) -> usize {
        self.map.len()
    }
    pub fn insert<S>(&mut self, k: S, v: V) -> Option<V>
    where
        S: Into<String>,
    {
        self.map.insert(k.into(), v)
    }
}

impl<'a, V: Default> Index<&'a String> for MutStrMap<V> {
    type Output = V;
    fn index(&self, idx: &String) -> &Self::Output {
        &self.map[idx.clone()]
    }
}
impl<'a, V: Default> Index<&'static str> for MutStrMap<V> {
    type Output = V;
    fn index(&self, idx: &'static str) -> &Self::Output {
        &self.map[idx.into()]
    }
}
impl<'a, V: Default> IndexMut<&'a String> for MutStrMap<V> {
    fn index_mut(&mut self, idx: &String) -> &mut Self::Output {
        &mut self.map[idx.clone()]
    }
}
impl<'a, V: Default> IndexMut<&'static str> for MutStrMap<V> {
    fn index_mut(&mut self, idx: &'static str) -> &mut Self::Output {
        &mut self.map[idx.into()]
    }
}
