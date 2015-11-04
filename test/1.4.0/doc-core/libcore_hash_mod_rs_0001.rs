fn main() {
    use std::hash::{Hash, Hasher, SipHasher};
    
    struct Person {
        id: u32,
        name: String,
        phone: u64,
    }
    
    impl Hash for Person {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.id.hash(state);
            self.phone.hash(state);
        }
    }
    
    let person1 = Person { id: 5, name: "Janet".to_string(), phone: 555_666_7777 };
    let person2 = Person { id: 5, name: "Bob".to_string(), phone: 555_666_7777 };
    
    assert_eq!(hash(&person1), hash(&person2));
    
    fn hash<T: Hash>(t: &T) -> u64 {
        let mut s = SipHasher::new();
        t.hash(&mut s);
        s.finish()
    }
}
