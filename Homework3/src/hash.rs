use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

struct HashMap<K, V> {
    buckets: Vec<Vec<(Rc<K>, Rc<V>)>>,
    size: usize,
}

impl<K: Eq + Hash, V> HashMap<K, V> {
    fn new() -> Self {
        let capacity = 16;
        HashMap {
            buckets: vec![Vec::new(); capacity],
            size: 0,
        }
    }

    fn hash<Q: ?Sized>(&self, key: &Q) -> u64
    where
        Q: Hash,
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }

    fn get_bucket_index<Q>(&self, key: &Q) -> usize
    where
        K: PartialEq<Q>,
    {
        let hash = self.hash(key);
        (hash as usize) % self.buckets.len()
    }

    fn insert(&mut self, key: K, value: V) {
        let key_rc = Rc::new(key);
        let value_rc = Rc::new(value);

        let bucket_index = self.get_bucket_index(&*key_rc);
        let bucket = &mut self.buckets[bucket_index];

        if let Some(existing) = bucket.iter_mut().find(|(existing_key, _)| Rc::ptr_eq(existing_key, &key_rc)) {
            *existing = (Rc::clone(&key_rc), Rc::clone(&value_rc));
        } else {
            bucket.push((Rc::clone(&key_rc), Rc::clone(&value_rc)));
            self.size += 1;
        }
    }

    fn get<Q>(&self, key: &Q) -> Option<Rc<V>>
    where
        K: PartialEq<Q>,
    {
        let key_rc = Rc::new(key);

        let bucket_index = self.get_bucket_index(&*key_rc);
        let bucket = &self.buckets[bucket_index];

        for &(ref existing_key, ref existing_value) in bucket.iter() {
            if Rc::ptr_eq(existing_key, &key_rc) {
                return Some(Rc::clone(existing_value));
            }
        }

        None
    }

    fn remove<Q>(&mut self, key: &Q) -> Option<Rc<V>>
    where
        K: PartialEq<Q>,
    {
        let key_rc = Rc::new(key);

        let bucket_index = self.get_bucket_index(&*key_rc);
        let bucket = &mut self.buckets[bucket_index];

        if let Some(index) = bucket.iter().position(|(existing_key, _)| Rc::ptr_eq(existing_key, &key_rc)) {
            let (_, value) = bucket.remove(index);
            self.size -= 1;
            Some(value)
        } else {
            None
        }
    }
}

fn main() {
    let mut hashmap: HashMap<&str, i32> = HashMap::new();
    hashmap.insert("one", 1);
    hashmap.insert("two", 2);
    hashmap.insert("three", 3);

    if let Some(value) = hashmap.get(&"two") {
        println!("Value for key 'two': {}", value);
    }

    if let Some(value) = hashmap.remove(&"one") {
        println!("Removed value for key 'one': {}", value);
    }
}
