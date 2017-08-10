use std::collections::HashMap;
use std::cell::RefCell;
use std::hash::Hash;
use std::collections::hash_map;
use std::ops::{Index, IndexMut};
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct MutMap<K: Eq + Hash, V: Default> {
    map: HashMap<K, RefCell<V>>,
}
pub struct Iter<'a, K: 'a, V: 'a + Default> {
    inner: hash_map::Iter<'a, K, RefCell<V>>,
}
pub struct Keys<'a, K: 'a, V: 'a + Default> {
    inner: Iter<'a, K, V>,
}
pub struct Values<'a, K: 'a, V: 'a + Default> {
    inner: Iter<'a, K, V>,
}


impl<'a, K, V: Default + Clone> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, V);
    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            None => None,
            Some((k, rv)) => {
                let v = rv.borrow().clone();
                Some((k, v))
            }
        }
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, K, V: Default + Clone> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, _)| k)
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, K, V: Default + Clone> Iterator for Values<'a, K, V> {
    type Item = V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(_, v)| v)
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}


impl<K: Eq + Hash + Clone + Debug, V: Default> Default for MutMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Eq + Hash + Clone + Debug, V: Default> MutMap<K, V> {
    pub fn new() -> Self {
        MutMap { map: HashMap::new() }
    }
    pub fn iter(&self) -> Iter<K, V> {
        Iter { inner: self.map.iter() }
    }
    pub fn len(&self) -> usize {
        self.map.len()
    }
    pub fn get(&self, idx: &K) -> Option<&V> {
        if self.map.contains_key(idx) {
            Some(&self.index(idx))
        } else {
            None
        }
    }
    pub fn contains_key(&self, idx: &K) -> bool {
        self.map.contains_key(idx)
    }
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        let result = self.map.insert(k, RefCell::new(v));
        match result {
            None => None,
            Some(v) => Some(v.into_inner()),
        }
    }
}

impl<'a, K: Hash + Eq + Clone + Debug, V: Default> Index<&'a K> for MutMap<K, V> {
    type Output = V;
    fn index(&self, idx: &K) -> &Self::Output {
        let map = &self.map;
        if !map.contains_key(idx) {
            panic!("{:?} not found", idx)
        }
        let cntp = map[idx].as_ptr();
        unsafe { &*cntp }
    }
}
impl<'a, K: Hash + Eq + Clone + Debug, V: Default> IndexMut<&'a K> for MutMap<K, V> {
    fn index_mut(&mut self, idx: &K) -> &mut Self::Output {
        let map = &mut self.map;
        if !map.contains_key(idx) {
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
    pub fn iter(&self) -> Iter<String, V> {
        self.map.iter()
    }
    pub fn keys(&self) -> Keys<String, V> {
        Keys { inner: self.iter() }
    }
    pub fn values(&self) -> Values<String, V> {
        Values { inner: self.iter() }
    }
    pub fn len(&self) -> usize {
        self.map.len()
    }
    pub fn get(&self, name: &str) -> Option<&V> {
        self.map.get(&name.into())
    }
    pub fn contains_key(&self, name: &str) -> bool {
        self.map.contains_key(&name.into())
    }

    pub fn insert(&mut self, k: &str, v: V) -> Option<V> {
        self.map.insert(k.into(), v)
    }
}

impl<'a, V: Default> Index<&'a String> for MutStrMap<V> {
    type Output = V;
    fn index(&self, idx: &String) -> &Self::Output {
        &self.map[idx]
    }
}
impl<'a, V: Default> Index<&'static str> for MutStrMap<V> {
    type Output = V;
    fn index(&self, idx: &'static str) -> &Self::Output {
        &self.map[&idx.into()]
    }
}
impl<'a, V: Default> IndexMut<&'a String> for MutStrMap<V> {
    fn index_mut(&mut self, idx: &String) -> &mut Self::Output {
        &mut self.map[idx]
    }
}
impl<'a, V: Default> IndexMut<&'static str> for MutStrMap<V> {
    fn index_mut(&mut self, idx: &'static str) -> &mut Self::Output {
        &mut self.map[&idx.into()]
    }
}
