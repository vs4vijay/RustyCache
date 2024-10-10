use std::collections::HashMap;

pub struct Cache<K, V> {
    map: HashMap<K, V>,
}

impl<K, V> Cache<K, V>
where
    K: std::cmp::Eq + std::hash::Hash,
{
    pub fn new() -> Self {
        Cache {
            map: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_get() {
        let mut cache = Cache::new();
        cache.add(1, "one");
        assert_eq!(cache.get(&1), Some(&"one"));
    }

    #[test]
    fn test_remove() {
        let mut cache = Cache::new();
        cache.add(1, "one");
        assert_eq!(cache.remove(&1), Some("one"));
        assert_eq!(cache.get(&1), None);
    }
}
