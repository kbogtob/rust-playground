use std::collections::{HashMap, VecDeque};

mod datastructures;

struct LRU {
    cache: HashMap<String, String>,
    use_order: VecDeque<String>,
    size: usize,
}

impl LRU {
    pub fn new(size: usize) -> LRU {
        LRU {
            cache: HashMap::new(),
            use_order: VecDeque::new(),
            size,
        }
    }

    pub fn set(&mut self, key: &str, value: String) -> &String {
        if self.use_order.len() == self.size {
            self.make_space();
        }
        self.cache.insert(String::from(key), value);
        self.use_order.push_back(String::from(key));
        &self.cache.get(key).unwrap()
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut String> {
        if self.cache.contains_key(key) {
            self.use_key(key);
        }

        self.cache.get_mut(key)
    }

    pub fn get(&mut self, key: &str) -> Option<&String> {
        if self.cache.contains_key(key) {
            self.use_key(key);
        }

        self.cache.get(key)
    }

    fn use_key(&mut self, key: &str) {
        if let Some(index) = self.use_order.iter().position(|x| *x == key) {
            self.use_order.remove(index);
        }

        self.use_order.push_back(String::from(key));
    }

    fn make_space(&mut self) {
        if let Some(key) = self.use_order.pop_front() {
            self.cache.remove(&key);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_keeps_items() {
        let mut cache = super::LRU::new(2);
        cache.set("a", String::from("xxx"));
        cache.set("b", String::from("yyy"));

        assert_eq!(cache.get("a"), Some(&"xxx".to_string()));
        assert_eq!(cache.get("b"), Some(&"yyy".to_string()));
    }

    #[test]
    fn it_cleans_items_after_set() {
        let mut cache = super::LRU::new(2);
        cache.set("a", String::from("xxx"));
        cache.set("b", String::from("yyy"));
        cache.set("c", String::from("zzz"));

        assert_eq!(cache.get("a"), None);
        assert_eq!(cache.get("b"), Some(&"yyy".to_string()));
        assert_eq!(cache.get("c"), Some(&"zzz".to_string()));
    }

    #[test]
    fn it_cleans_items_after_get() {
        let mut cache = super::LRU::new(2);
        cache.set("a", String::from("xxx"));
        cache.set("b", String::from("yyy"));
        cache.get("a");
        cache.set("c", String::from("zzz"));

        assert_eq!(cache.get("a"), Some(&"xxx".to_string()));
        assert_eq!(cache.get("b"), None);
        assert_eq!(cache.get("c"), Some(&"zzz".to_string()));
    }

    #[test]
    fn it_cleans_items_after_get_mut() {
        let mut cache = super::LRU::new(2);
        cache.set("a", String::from("xxx"));
        cache.set("b", String::from("yyy"));
        cache.get_mut("a");
        cache.set("c", String::from("zzz"));

        assert_eq!(cache.get("a"), Some(&"xxx".to_string()));
        assert_eq!(cache.get("b"), None);
        assert_eq!(cache.get("c"), Some(&"zzz".to_string()));
    }
}
