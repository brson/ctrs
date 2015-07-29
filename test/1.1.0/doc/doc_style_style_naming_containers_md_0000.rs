fn main() {
    // Look up element without failing
    fn get(&self, key: K) -> Option<&V>
    fn get_mut(&mut self, key: K) -> Option<&mut V>
    
    // Convenience for .get(key).map(|elt| elt.clone())
    fn get_clone(&self, key: K) -> Option<V>
    
    // Lookup element, failing if it is not found:
    impl Index<K, V> for Container { ... }
    impl IndexMut<K, V> for Container { ... }
}
